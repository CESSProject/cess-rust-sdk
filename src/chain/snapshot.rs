use super::audit::Audit;
use super::ChainSdk;
use crate::core::pattern::ChallengeInfo;
use crate::utils::account_from_slice;
use anyhow::Result;
use async_trait::async_trait;
use subxt::ext::sp_core::H256;

#[async_trait]
pub trait Snapshot {
    async fn query_challenge(&self, pk: &[u8], block_hash: Option<H256>) -> Result<ChallengeInfo>;
}

#[async_trait]
impl Snapshot for ChainSdk {
    async fn query_challenge(&self, pk: &[u8], block_hash: Option<H256>) -> Result<ChallengeInfo> {
        let netinfo = self.query_challenge_snapshot(block_hash).await?;
        let account = account_from_slice(pk);

        let mut chal = ChallengeInfo {
            ..Default::default()
        };

        if let Some(netinfo) = netinfo {
            for v in &netinfo.miner_snapshot_list.0 {
                if v.miner == account {
                    for (k, value) in netinfo.net_snap_shot.random_list.0.iter().enumerate() {
                        chal.random[k] = hex::encode(value).into_bytes();
                        chal.random_index_list[k] = netinfo.net_snap_shot.random_index_list.0[k];
                    }
                    chal.start = netinfo.net_snap_shot.start;
                    break;
                }
            }
        }
        Ok(chal)
    }
}

#[cfg(test)]
mod test {
    use super::Snapshot;
    use crate::{chain::ChainSdk, core::utils::account::parsing_public_key};
    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    fn init_chain() -> ChainSdk {
        ChainSdk::new(MNEMONIC, "service_name")
    }

    #[tokio::test]
    pub async fn test_query_challenge() {
        let sdk = init_chain();
        let account_address = "cXjmuHdBk4J3Zyt2oGodwGegNFaTFPcfC48PZ9NMmcUFzF6cc";
        let pk_bytes = parsing_public_key(account_address).unwrap();
        let result = sdk.query_challenge(&pk_bytes, None).await;
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(e) => {
                println!("error: {:?}", e);
                assert!(false);
            }
        }
    }
}
