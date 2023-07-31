use anyhow::Result;

pub trait SDK {
    // Audit-State

    // Query Challenge Expired Block Height Interface
    fn query_challenge_expiration() -> Result<u32>;

    // Query Challenge Snapshot query challenge information snapshot.
}
