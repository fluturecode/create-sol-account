use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
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
  pub fn create_account(owner: &Pubkey, lamports: u64, program_id: &Pubkey) -> MyAccount {
      let account = solana_sdk::account::Account::new(lamports, MyAccount::LEN, program_id);
      let mut data = vec![0u8; MyAccount::LEN];
      let my_string = "Hello World".to_string();
      let my_u8 = 42;
      let my_account = MyAccount {
          my_string,
          my_u8,
      };
      my_account.pack_into_slice(&mut data);
      solana_sdk::program::invoke_signed(
          &solana_sdk::system_instruction::create_account(
              owner,
              &account.pubkey(),
              lamports,
              MyAccount::LEN as u64,
              program_id,
          ),
          &[&account, &owner],
          &[&data],
      )
      .unwrap();

      MyAccount::