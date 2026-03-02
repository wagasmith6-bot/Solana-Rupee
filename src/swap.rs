use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use meteora_damm_v2_sdk::instruction::swap;

pub fn build_swap(
    pool: Pubkey,
    amount_in: u64,
    min_amount_out: u64,
) -> Instruction {
    swap::create_swap_instruction(pool, amount_in, min_amount_out)
}

// Usage: Embed in a signed transaction and send via Jito bundle