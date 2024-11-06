### Examples

Examples showing how to use the CESS Rust SDK.

### Setup

1. Create .env file and create the following two variables.
    ```
    RPC_NETWORK=<mainnet | testnet>
    RPC_URL=<RPC_URL>
    ```
    *Note:* The API first tries to connect using the custom RPC_URL, if fail to connect it will try to connect to default public RPC node(mainnet/testnet) depending on the value set for RPC_NETWORK.

2. Depending on which pallet you want to access, import the modules as needed.
    ```
    // StorageTransaction for balances crate.
    use cess_rust_sdk::chain::balances::transaction::StorageTransaction;

    // StorageQuery for storage_handler crate.
    use cess_rust_sdk::chain::storage_handler::query::StorageQuery;
    ```