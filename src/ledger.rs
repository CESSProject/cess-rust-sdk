use std::sync::Arc;

use super::core::Error;
use ledger_apdu::APDUAnswer;
use ledger_transport::APDUCommand;
use ledger_transport_hid::{hidapi::HidApi, TransportNativeHID};
use subxt::{
    ext::sp_core::sr25519,
    tx::Signer as SubxtSignerTrait,
    utils::{AccountId32, MultiAddress, MultiSignature},
    Config, PolkadotConfig,
};

const APDU_CHUNK_SIZE: usize = 230;

pub struct LedgerSigner {
    _hidapi: Arc<HidApi>,
    transport: TransportNativeHID,
    account_id: AccountId32,
    derivation_path: String,
}

impl LedgerSigner {
    pub fn new(derivation_path: &str) -> Result<Self, Error> {
        let hid = HidApi::new().map_err(Error::Hid)?;
        let hid_arc = Arc::new(hid);

        let transport =
            TransportNativeHID::new(&hid_arc).map_err(|e| Error::Transport(format!("{:?}", e)))?;

        let pubkey = Self::send_get_address_sync(&transport, derivation_path)
            .map_err(|e| Error::APDU(format!("{:?}", e)))?;

        if pubkey.len() != 32 {
            return Err(Error::BadResponse);
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&pubkey[..32]);

        let account_id = AccountId32::from(arr);
        Ok(Self {
            _hidapi: hid_arc,
            transport,
            account_id,
            derivation_path: derivation_path.to_string(),
        })
    }

    fn send_get_address_sync(
        transport: &TransportNativeHID,
        derivation_path: &str,
    ) -> Result<Vec<u8>, Error> {
        let path_bytes: Vec<u8> = pack_bip44_path_bytes(derivation_path)?;

        let cla: u8 = 0xF9;
        let ins_get_addr: u8 = 0x01;
        let cmd = APDUCommand {
            cla,
            ins: ins_get_addr,
            p1: 0x00,
            p2: 0x00,
            data: path_bytes,
        };

        let answer: APDUAnswer<Vec<u8>> = transport
            .exchange(&cmd)
            .map_err(|e| Error::Transport(format!("transport.exchange failed: {:?}", e)))?;

        let sw = answer.retcode();
        if sw != 0x9000 {
            return Err(Error::APDU(format!("apdu returned sw {:04x}", sw)));
        }

        let data = answer.apdu_data().to_vec();
        Ok(data)
    }

    fn send_sign_sync(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let path_bytes = pack_bip44_path_bytes(&self.derivation_path)?;
        // INIT APDU: path bytes + message length (u16 BE) — adjust per app
        let mut init_payload = path_bytes;
        init_payload.extend_from_slice(&(message.len() as u16).to_be_bytes());
        let init_cmd = APDUCommand {
            cla: 0xF9,
            ins: 0x02,
            p1: 0x00,
            p2: 0x00,
            data: init_payload,
        };
        let _ = self
            .transport
            .exchange(&init_cmd)
            .map_err(|e| format!("{:?}", e))?;

        let mut offset = 0usize;
        let mut sig: Option<Vec<u8>> = None;
        while offset < message.len() {
            let end = (offset + APDU_CHUNK_SIZE).min(message.len());
            let chunk = &message[offset..end];
            let p1 = if end == message.len() { 0x02 } else { 0x01 };
            let cmd = APDUCommand {
                cla: 0xF9,
                ins: 0x02,
                p1,
                p2: 0x00,
                data: chunk.to_vec(),
            };
            let ans = self
                .transport
                .exchange(&cmd)
                .map_err(|e| format!("{:?}", e))?;
            if p1 == 0x02 {
                sig = Some(ans.apdu_data().to_vec());
            }
            offset = end;
        }
        sig.ok_or_else(|| "no signature returned".to_string())
    }
}

fn pack_bip44_path_bytes(path: &str) -> Result<Vec<u8>, String> {
    let s = path.trim().strip_prefix("m/").unwrap_or(path);
    let parts: Vec<&str> = s.split("/").collect();
    if parts.is_empty() {
        return Err("empty path".into());
    }
    let mut out = Vec::with_capacity(1 + parts.len() * 4);
    out.push(parts.len() as u8);
    for p in parts {
        let hardened = p.ends_with('\'') || p.ends_with('h');
        let num_str = if hardened { &p[..p.len() - 1] } else { p };
        let v: u32 = num_str
            .parse()
            .map_err(|e| format!("bad path component: {}", e))?;
        let val = if hardened { v | 0x8000_0000 } else { v };
        out.extend_from_slice(&val.to_be_bytes());
    }

    Ok(out)
}

/// Implement the Subxt Signer trait so this can be boxed into `AnySigner`.
impl SubxtSignerTrait<PolkadotConfig> for LedgerSigner {
    fn account_id(&self) -> <PolkadotConfig as Config>::AccountId {
        self.account_id.clone()
    }

    fn address(&self) -> <PolkadotConfig as Config>::Address {
        // Return the AccountId32 as the Address (works for PolkadotConfig)
        MultiAddress::Id(self.account_id.clone())
    }

    fn sign(&self, payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
        let sig_bytes = self
            .send_sign_sync(payload)
            .unwrap_or_else(|e| panic!("Ledger sign failed: {}", e));

        if sig_bytes.len() < 64 {
            panic!("signature too short from ledger");
        }
        let mut sig64 = [0u8; 64];
        sig64.copy_from_slice(&sig_bytes[0..64]);
        let sr_sig = sr25519::Signature::from_raw(sig64);
        MultiSignature::Sr25519(sr_sig.into())
    }
}
