use solana_sdk::system_instruction;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyAccount {
    pub my_string: String,
    pub my_u8: u8,
}

impl Sealed for MyAccount {}

impl IsInitialized for MyAccount {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for MyAccount {
    const LEN: usize = 1 + 1 + 256; // 1 byte for u8, 1 byte for string length, and 256 bytes for string data

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut offset = 0;
        dst[offset] = self.my_u8;
        offset += 1;

        let string_bytes = self.my_string.as_bytes();
        let string_len: u8 = string_bytes.len().try_into().unwrap();
        dst[offset] = string_len;
        offset += 1;

        dst[offset..offset + string_bytes.len()].copy_from_slice(string_bytes);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut offset = 0;
        let my_u8 = src[offset];
        offset += 1;

        let string_len = src[offset];
        offset += 1;

        let string_data = &src[offset..offset + string_len as usize];
        offset += string_len as usize;

        let my_string = String::from_utf8_lossy(string_data).to_string();

        Ok(MyAccount { my_string, my_u8 })
    }
}

impl MyAccount {
    pub fn create_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        my_u8: u8,
        my_string: String,
        lamports: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;

        if !account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let my_account = MyAccount { my_u8, my_string };

        if !my_account.is_initialized() {
            let rent_exempt_reserve = rent.minimum_balance(MyAccount::LEN);
            if lamports < rent_exempt_reserve {
                return Err(ProgramError::InsufficientFunds);
            }

            let ix = system_instruction::create_account(
                system_program_info.key,
                &account.key,
                lamports,
                MyAccount::LEN as u64,
                program_id,
            );

            solana_program::program::invoke(&ix, &[account.clone(), system_program_info.clone()])?;

            my_account.pack_into_slice(&mut account.data.borrow_mut());
        }

        if owner_info.key != account.owner {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(())
    }
}
