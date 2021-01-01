#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod game {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap;
    use alloc::collections::BTreeMap;
    
    #[ink(storage)]
    pub struct Game {
        game_accounts: HashMap<AccountId, GameAccount>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct GameAccount {
        level: u32,
        level_programs: BTreeMap<u32, AccountId>,
        // todo: other data for a captain
        // e.g.: NFT pet, Erc20 gold in game
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
        pub fn run_level(&mut self, level: u32, program_id: AccountId) -> Result<(), Error> {
            let caller = self.env().caller();
            // todo: veriy caller's program by calling its function

            Ok(())
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
    
    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
