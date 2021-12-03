use borsh::{BorshDeserialize, BorshSerialize};
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
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
  /// number of greetings
  pub counter: u32,
  pub random: u32,
}

use switchboard_program::VrfAccount;

fn my_hash<T>(obj: T) -> u64
where
  T: Hash,
{
  let mut hasher = DefaultHasher::new();
  obj.hash(&mut hasher);
  hasher.finish()
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
// yorkid account public key: CR7VpPEW21ZtBYp4BbSypootPHZvXh2LQSR9jDXaN5ba, program id: DW6HET1JQjkMsszJK1pXdJWhs6enSYBgyMXq9zz4qtzP
// york url https://explorer.solana.com/address/DW6HET1JQjkMsszJK1pXdJWhs6enSYBgyMXq9zz4qtzP?cluster=devnet
// Program entrypoint's implementation
fn process_instruction<'a>(
  program_id: &Pubkey,
  accounts: &'a [AccountInfo<'a>],
  instruction_data: &[u8],
) -> ProgramResult {
  msg!("york0");
  let accounts_iter = &mut accounts.iter();
  msg!("york1");
  let lottery_data_account = next_account_info(accounts_iter)?;
  msg!("york2");
  let vrf_account_info = next_account_info(accounts_iter)?;
  msg!("york3");
  msg!("{:?}", vrf_account_info.key);
  let vrf_account = VrfAccount::new(vrf_account_info)?;
  msg!("york4");
  let random_numbers = vrf_account.get_verified_randomness()?;
  msg!("york5");

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
      GreetingAccount::try_from_slice(&accounts[0].data.borrow())
        .unwrap()
        .counter,
      0
    );
    process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    assert_eq!(
      GreetingAccount::try_from_slice(&accounts[0].data.borrow())
        .unwrap()
        .counter,
      1
    );
    process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    assert_eq!(
      GreetingAccount::try_from_slice(&accounts[0].data.borrow())
        .unwrap()
        .counter,
      2
    );
  }
}
