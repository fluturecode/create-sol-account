use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instructions);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Your program logic goes here
    msg!("Program entrypoint");

    // Parse the instruction data
    let instruction = MyInstruction::unpack(instruction_data)?;

    // Get the account that will store the data
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Create a new account and store the data
    let my_account = MyAccount::new();
    my_account.serialize(&mut *account.data.borrow_mut())?;

    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct MyInstruction {
    pub data: [u8; 8],
}

impl MyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let data = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(MyInstruction { data })
    }
}

// Define your account data structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyAccount {
    pub my_string: String,
    pub my_u8: u8,
}

impl MyAccount {
    pub fn new() -> Self {
        MyAccount {
            my_string: "Hello World".to_string(),
            my_u8: 42,
        }
    }

    pub fn serialize(&self, output: &mut [u8]) -> ProgramResult {
        let data = (self.my_string.clone(), self.my_u8);
        let packed_data =
            bincode::serialize(&data).map_err(|_| ProgramError::InvalidAccountData)?;
        output.copy_from_slice(&packed_data);
        Ok(())
    }

    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        let (my_string, my_u8) =
            bincode::deserialize(input).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(MyAccount { my_string, my_u8 })
    }
}
