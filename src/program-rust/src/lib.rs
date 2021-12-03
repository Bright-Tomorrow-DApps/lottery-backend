extern crate easy_hasher;
use borsh::{BorshDeserialize, BorshSerialize};
use easy_hasher::easy_hasher::*;
use sha2::{Digest, Sha256};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Command {
    pub counter: u32,
    pub lottery_content: String,
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RandomAccount {
    /// number of greetings
    pub counter: u32,
    pub random: u32,
}

fn data_to_hash_number<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
// program info:
// * url: https://explorer.solana.com/address/DW6HET1JQjkMsszJK1pXdJWhs6enSYBgyMXq9zz4qtzP?cluster=devnet
// * public key: CR7VpPEW21ZtBYp4BbSypootPHZvXh2LQSR9jDXaN5ba
// * program id: DW6HET1JQjkMsszJK1pXdJWhs6enSYBgyMXq9zz4qtzP
// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Start random program entrypoint");
    let mut instruction_data_mut = instruction_data;
    let command = Command::deserialize(&mut instruction_data_mut)?;
    let accounts_iter = &mut accounts.iter();
    let clock_via_sysvar = Clock::get()?;
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let hash = sha256(&command.lottery_content);
    let hash_lottery_content = hash.to_hex_string();

    msg!("content size: {:?}", command.counter);
    msg!("hash lottery content: {:?}", hash_lottery_content);
    msg!(
        "clock timestamp via sysvar: {:?}",
        clock_via_sysvar.unix_timestamp
    );
    msg!("clock slot via sysvar:  {:?}", clock_via_sysvar.slot);

    let u64_command_counter = u64::try_from(command.counter).unwrap();
    let u64_random_number = (data_to_hash_number(clock_via_sysvar.slot) % u64_command_counter) + 1;
    let u32_random_number = u32::try_from(u64_random_number).unwrap();
    msg!("u64 random number: {:?}", u64_random_number);
    msg!("u32 random number: {:?}", u32_random_number);

    let mut random_account = RandomAccount::try_from_slice(&account.data.borrow())?;
    random_account.random = u32_random_number;
    random_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            RandomAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            RandomAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            RandomAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
