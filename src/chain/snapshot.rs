// use crate::chain::audit;
use super::Sdk;
use crate::core::pattern::ChallengeInfo;
use crate::utils::account_from_slice;
use anyhow::Result;

impl Sdk {
    pub async fn query_challenge(&self, pk: &[u8]) -> Result<ChallengeInfo> {
        let netinfo = self.query_challenge_snapshot().await?;
        let account = account_from_slice(pk);

        let mut chal = ChallengeInfo {
            ..Default::default()
        };

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
        Ok(chal)
    }
}

#[cfg(test)]
mod test {
    use crate::chain::Sdk;
    const MNEMONIC: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

    fn init_sdk() -> Sdk {
        Sdk::new(MNEMONIC, "service_name",  true)
    }

    #[tokio::test]
    pub async fn test_query_challenge() {
        let sdk = init_sdk();
        let pk = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let pk_bytes = hex::decode(pk).unwrap();
        let result = sdk.query_challenge(&pk_bytes).await;
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
