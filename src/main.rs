use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::processor::process_instruction;

entrypoint!(process_instruction);

pub fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let owner = next_account_info(account_info_iter)?;
    let account = next_account_info(account_info_iter)?;

    let mut data = vec![0u8; MyAccount::LEN];
    let my_string = "Hello World".to_string();
    let my_u8 = 42;
    let my_account = MyAccount { my_string, my_u8 };
    my_account.pack_into_slice(&mut data);

    let ix = system_instruction::create_account(
        owner.key,
        account.key,
        lamports,
        MyAccount::LEN as u64,
        program_id,
    );

    solana_program::program::invoke_signed(&ix, &[owner.clone(), account.clone()], &[&data])?;

    Ok(())
}
