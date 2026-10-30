mod pb;

use pb::solana::transfers::v1::{Transfer, Transfers};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const SYSTEM_TRANSFER_DISCRIMINANT: u32 = 2;
const SYSTEM_TRANSFER_WITH_SEED_DISCRIMINANT: u32 = 11;

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<Transfers, substreams::errors::Error> {
    let mut transfers = Vec::new();

    for confirmed_trx in block.transactions {
        let Some(meta) = &confirmed_trx.meta else { continue };
        let Some(transaction) = &confirmed_trx.transaction else { continue };

        // Skip failed transactions
        if meta.err.is_some() {
            continue;
        }

        let Some(message) = &transaction.message else { continue };

        // Build complete resolved account list (static + ALT addresses)
        let mut account_keys: Vec<String> = message
            .account_keys
            .iter()
            .map(|k| bs58::encode(k).into_string())
            .collect();

        for loaded in &meta.loaded_writable_addresses {
            account_keys.push(bs58::encode(loaded).into_string());
        }
        for loaded in &meta.loaded_readonly_addresses {
            account_keys.push(bs58::encode(loaded).into_string());
        }

        // Top-level instructions
        for inst in &message.instructions {
            if let Some(t) = extract_system_transfer(
                inst.program_id_index, &inst.data, &inst.accounts, &account_keys,
            ) {
                transfers.push(t);
            }
        }

        // Inner (CPI) instructions
        for inner in &meta.inner_instructions {
            for inst in &inner.instructions {
                if let Some(t) = extract_system_transfer(
                    inst.program_id_index, &inst.data, &inst.accounts, &account_keys,
                ) {
                    transfers.push(t);
                }
            }
        }
    }

    Ok(Transfers { transfers })
}

fn extract_system_transfer(
    program_id_index: u32,
    data: &[u8],
    accounts: &[u8],
    account_keys: &[String],
) -> Option<Transfer> {
    if account_keys.get(program_id_index as usize)?.as_str() != SYSTEM_PROGRAM_ID {
        return None;
    }
    if data.len() < 4 {
        return None;
    }

    let type_index = u32::from_le_bytes(data[0..4].try_into().ok()?);

    match type_index {
        SYSTEM_TRANSFER_DISCRIMINANT => {
            if data.len() < 12 || accounts.len() < 2 { return None; }
            let lamports = u64::from_le_bytes(data[4..12].try_into().ok()?);
            Some(Transfer {
                from: account_keys.get(accounts[0] as usize)?.clone(),
                to:   account_keys.get(accounts[1] as usize)?.clone(),
                amount: lamports,
            })
        }
        SYSTEM_TRANSFER_WITH_SEED_DISCRIMINANT => {
            if data.len() < 8 || accounts.len() < 3 { return None; }
            let lamports = u64::from_le_bytes(data[data.len()-8..].try_into().ok()?);
            Some(Transfer {
                from: account_keys.get(accounts[0] as usize)?.clone(),
                to:   account_keys.get(accounts[2] as usize)?.clone(),
                amount: lamports,
            })
        }
        _ => None,
    }
}
