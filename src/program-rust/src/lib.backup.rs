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
pub fn process_instruction(
  program_id: &Pubkey, // Public key of the account the hello world program was loaded into
  accounts: &[AccountInfo], // The account to say hello to
  instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
  let mut instruction_data_mut = instruction_data;
  msg!("Hello World Rust program entrypoint");
  let command = Command::deserialize(&mut instruction_data_mut)?;
  msg!("york000");
  msg!("{:?}", command.counter);
  msg!("york0");
  // Iterating accounts is safer then indexing
  let accounts_iter = &mut accounts.iter();
  msg!("york01123");
  // Get the clock sysvar via syscall
  msg!("york019999");
  let clock_via_sysvar = Clock::get()?;
  // msg!("york01");
  // Or deserialize the account into a clock struct
  // let clock_sysvar_info = next_account_info(accounts_iter)?;
  // msg!("york02");
  // let clock_via_account = Clock::from_account_info(clock_sysvar_info)?;
  // Both produce the same sysvar
  // assert_eq!(clock_via_sysvar, clock_via_account);
  // Note: `format!` can be very expensive, use cautiously
  msg!("york3");
  msg!("{:?}", clock_via_sysvar.unix_timestamp);
  msg!("york3123");
  msg!("{:?}", clock_via_sysvar.slot);
  msg!("york1");
  // msg!(
  //     "{:x}",
  //     Sha256::digest(clock_via_sysvar.unix_timestamp.to_string().as_bytes())
  // );
  let u32_slot = u32::try_from(clock_via_sysvar.slot).unwrap();
  let u64_command_counter = u64::try_from(command.counter).unwrap();
  let u64_ans = (my_hash(u32_slot) % u64_command_counter) + 1;
  let ans = u32::try_from(u64_ans).unwrap();
  msg!("york145678");
  msg!("{:?}", u32_slot);
  msg!("york2");
  msg!("{:?}", my_hash(u32_slot));
  msg!("york222223");
  msg!("{:?}", u64_ans);
  msg!("york233333");
  msg!("{:?}", ans);
  msg!("york5555");
  // msg!("{:?}", clock_via_account);
  // Get the account to say hello to
  let account = next_account_info(accounts_iter)?;

  // The account must be owned by the program in order to modify its data
  if account.owner != program_id {
    msg!("Greeted account does not have the correct program id");
    return Err(ProgramError::IncorrectProgramId);
  }

  // Increment and store the number of times the account has been greeted
  let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
  greeting_account.counter += 1;
  greeting_account.random = ans;
  greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

  msg!("Greeted {} time(s)!", greeting_account.counter);

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
