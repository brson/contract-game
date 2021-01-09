#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod game {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap;
    use alloc::collections::BTreeMap;
    use alloc::format;
    use ink_env::call::Selector;
    use ink_env::call::build_call;
    use ink_env::DefaultEnvironment;
    use ink_env::call::ExecutionInput;
    use ink_env::call::utils::ReturnType;
    use core::convert::TryFrom;
    
    #[ink(storage)]
    pub struct Game {
        game_accounts: HashMap<AccountId, GameAccount>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct GameAccount {
        level: u32,
        level_programs: BTreeMap<u32, AccountId>,
    }

    impl GameAccount {
        pub fn default() -> Self {
            GameAccount {
                level: 0,
                level_programs: BTreeMap::new(),
            }
        }
    }
    
    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub enum Error {
        InsufficiantBalance,
        AccountExists,
        AccountNotExists,
        SubmitProgramFailed,
        SubmittedGreaterLevel,
        ProgramNotExists,
    }

    // Contract methods
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game {
                game_accounts: HashMap::new(),
            }
        }

        /// Query if the caller has an account
        #[ink(message)]
        pub fn have_game_account(&self, account: AccountId) -> bool {
            self.game_accounts.contains_key(&account)
        }

        /// Create an account for the caller
        ///
        /// # Errors
        ///
        /// - The account exists.
        /// - The paid amount is insufficient.
        #[ink(message, payable)]
        pub fn create_game_account(&mut self) -> Result<GameAccount, Error> {
            let caller = self.env().caller();
            let balance = self.env().transferred_balance();

            // create a captain costs 1000 money?
            // todo: return transferred_balance on Error?
            if balance < 1000 {
                ink_env::debug_println("Your balance isn't enough for creating your captain");
                Err(Error::InsufficiantBalance)
            } else if self.have_game_account(caller) {
                Err(Error::AccountExists)
            } else {
                self.create_a_captain(caller)
            }           
        }

        /// Retrieve caller's account information
        ///
        /// # Errors
        ///
        /// - The account doesn't exist.
        #[ink(message)]
        pub fn get_game_account(&self, account: AccountId) -> Result<GameAccount, Error> {
            self.game_accounts.get(&account).cloned().ok_or(Error::AccountNotExists)
        }

        /// Submit a program for a level puzzle
        ///
        /// # Errors
        ///
        /// - Level is greater than caller's current level.
        /// - Program fails verification.
        /// - Program account doesn't exist.
        #[ink(message, payable)]
        pub fn submit_level(&mut self, level: u32, program_id: AccountId) -> Result<AccountId, Error> {
            // todo: verify transferred balance
            let caller = self.env().caller();
            
            if let Some(game_account) = self.game_accounts.get_mut(&caller) {
                let account_current_level = game_account.level;
                if level <= account_current_level {
                    game_account.level_programs.insert(level, program_id).ok_or(Error::SubmitProgramFailed)
                } else {
                    Err(Error::SubmittedGreaterLevel)
                }
            } else {
                Err(Error::AccountNotExists)
            }
        }

        /// Run a level
        ///
        /// # Errors
        ///
        /// - Level is greater than caller's current level.
        /// - Caller has no submiss for this level.
        /// - Submitted program doesn't implement required contracts.
        #[ink(message, payable)]
        pub fn run_level(&mut self, level: u32) -> Result<bool, Error> {
            let caller = self.env().caller();
            if let Some(game_account) = self.game_accounts.get_mut(&caller) {
                ink_env::debug_println(&format!("game account: {:?}", game_account));
                if let Some(program_id) = game_account.level_programs.get(&level) {
                    ink_env::debug_println(&format!("program id: {:?}", program_id));
                    dispatch_level(level, program_id.clone())                    
                } else {
                    Err(Error::ProgramNotExists)
                }
            } else {
                Err(Error::AccountNotExists)
            }        
        }

        #[ink(message, payable)]
        pub fn run_level_test_2(&mut self, program_id: AccountId) -> Result<bool, Error> {
            dispatch_level(5, program_id)
        }

        #[ink(message, payable)]
        pub fn run_level_test(&mut self) -> bool {
            let program_id = "4cfac7f74c6233449b5e54ba070231dd94c71b89505482cd910000656258d3ed";
            ink_env::debug_println(&format!("hash {:?}", program_id));
            
            let program_id = hex::decode(program_id).unwrap();
            ink_env::debug_println(&format!("decode {:?}", program_id));
            
            let program_id = AccountId::try_from(&program_id[..]).unwrap();
            ink_env::debug_println(&format!("AccountId {:?}", program_id));

            true
        }
    }

    // Methos support contracts
    impl Game {
        fn create_a_captain(&mut self, account: AccountId) -> Result<GameAccount, Error> {
            let new_game_account = GameAccount::default();

            self.game_accounts.insert(account, new_game_account.clone());
            Ok(new_game_account)
        }
    }

    fn dispatch_level(level: u32, program_id: AccountId) -> Result<bool, Error> {
        /*let program_id = "4cfac7f74c6233449b5e54ba070231dd94c71b89505482cd910000656258d3ed";
        ink_env::debug_println(&format!("hash {:?}", program_id));
        
        let program_id = hex::decode(program_id).unwrap();
        ink_env::debug_println(&format!("decode {:?}", program_id));
       
        let program_id = AccountId::try_from(&program_id[..]).unwrap();
        ink_env::debug_println(&format!("AccountId {:?}", program_id));*/

        ink_env::debug_println(&format!("calling flip on {:?}", program_id));
        let return_value: () = build_call::<DefaultEnvironment>()
            .callee(program_id) 
            .gas_limit(50)
            .transferred_value(10)
            .exec_input(
                ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF]))
            )
            .returns::<ReturnType<()>>()
            .fire()
            .unwrap();

        ink_env::debug_println(&format!("calling get on {:?}", program_id));
        let return_value: bool = build_call::<DefaultEnvironment>()
            .callee(program_id) 
            .gas_limit(50)
            .transferred_value(10)
            .exec_input(
                ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
            )
            .returns::<ReturnType<bool>>()
            .fire()
            .unwrap();
        
        ink_env::debug_println(&format!("return value {}", return_value));
        Ok(return_value)
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
