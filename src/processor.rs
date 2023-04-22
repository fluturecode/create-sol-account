use crate::{instruction::MyProgramInstruction, state::MyAccount};

pub mod processor {
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
    };

    pub struct Processor;

    impl Processor {
        pub fn process(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            instruction_data: &[u8],
        ) -> ProgramResult {
            let instruction = MyProgramInstruction::unpack(instruction_data)?;

            match instruction {
                MyProgramInstruction::CreateMyAccount { my_u8, my_string } => {
                    Self::process_create_my_account(program_id, accounts, my_u8, my_string)
                }
            }
        }

        fn process_create_my_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            my_u8: u8,
            my_string: String,
        ) -> ProgramResult {
            let account_info_iter = &mut accounts.iter();

            let account = next_account_info(account_info_iter)?;

            MyAccount::create_account(program_id, accounts, my_u8, my_string, account.lamports())
        }
    }
}
