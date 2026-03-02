use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub fn get_vault_balance(
    rpc: &RpcClient,
    vault: &Pubkey,
) -> f64 {
    let bal = rpc.get_token_account_balance(vault).unwrap();
    bal.amount.parse::<f64>().unwrap()
}

// Example usage with mainnet:
// let rpc = RpcClient::new(env::var("HELIUS_RPC").unwrap());