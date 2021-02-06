#![allow(unused)]
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
    
    #[ink(storage)]
    pub struct Game {
        player_accounts: HashMap<AccountId, PlayerAccount>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct PlayerAccount {
        level: u32,
        level_contracts: BTreeMap<u32, AccountId>,
    }

    impl PlayerAccount {
        pub fn default() -> Self {
            PlayerAccount {
                level: 0,
                level_contracts: BTreeMap::new(),
            }
        }
    }
    
    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub enum Error {
        InsufficiantBalance,
        AccountExists,
        AccountNotExists,
        SubmitLevelContractFailed,
        SubmittedGreaterLevel,
        LevelContractNotExists,
        LevelContractCallFailed,
    }

    // Contract methods
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game {
                player_accounts: HashMap::new(),
            }
        }

        /// Query if the caller has an account
        #[ink(message)]
        pub fn have_player_account(&self, account: AccountId) -> bool {
            self.player_accounts.contains_key(&account)
        }

        /// Create an account for the caller
        ///
        /// # Errors
        ///
        /// - The account exists.
        /// - The paid amount is insufficient.
        #[ink(message)]
        pub fn create_player_account(&mut self) -> Result<PlayerAccount, Error> {
            let caller = self.env().caller();

            if self.have_player_account(caller) {
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
        pub fn get_player_account(&self, account: AccountId) -> Result<PlayerAccount, Error> {
            self.player_accounts.get(&account).cloned().ok_or(Error::AccountNotExists)
        }

        /// Submit a program for a level puzzle
        ///
        /// # Errors
        ///
        /// - Level is greater than caller's current level.
        /// - Program fails verification.
        /// - Program account doesn't exist.
        #[ink(message)]
        pub fn submit_level(&mut self, level: u32, level_contract: AccountId) -> Result<AccountId, Error> {
            let caller = self.env().caller();
            
            if let Some(player_account) = self.player_accounts.get_mut(&caller) {
                let account_current_level = player_account.level;
                if level <= account_current_level {
                    ink_env::debug_println(&format!("insert level {}, and contract {:?}", level, level_contract));
                    player_account.level_contracts.insert(level, level_contract).ok_or(Error::SubmitLevelContractFailed)
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
        #[ink(message)]
        pub fn run_level(&mut self, level: u32) -> Result<bool, Error> {
            let caller = self.env().caller();
            if let Some(player_account) = self.player_accounts.get_mut(&caller) {
                ink_env::debug_println(&format!("game account: {:?}", player_account));

                if let Some(level_contract) = player_account.level_contracts.get(&level) {
                    ink_env::debug_println(&format!("program id: {:?}", level_contract));
                    dispatch_level(level, level_contract.clone())                    
                } else {
                    Err(Error::LevelContractNotExists)
                }
            } else {
                Err(Error::AccountNotExists)
            }        
        }

    }

    // Non-contract support methods
    impl Game {
        fn create_a_captain(&mut self, account: AccountId) -> Result<PlayerAccount, Error> {
            let new_player_account = PlayerAccount::default();
            self.player_accounts.insert(account, new_player_account.clone());
            ink_env::debug_println(&format!("new player account {:?}", new_player_account));

            Ok(new_player_account)
        }
    }

    fn dispatch_level(level: u32, level_contract: AccountId) -> Result<bool, Error> {
        ink_env::debug_println(&format!("dispatch level: {}, calling contract: {:?}", level, level_contract));

        let selector;
        match level {
            0 => selector = [0xDE, 0xAD, 0xBE, 0xEF],
            1 => selector = [0xDE, 0xAD, 0xEE, 0xEE],
            _ => unreachable!(),
        }
        
        ink_env::debug_println(&format!("contract selector: {:?}", &selector));

        let return_value = build_call::<DefaultEnvironment>()
            .callee(level_contract)
            .exec_input(
                ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF]))
                // ExecutionInput::new(Selector::new(selector))  
            )
            .returns::<ReturnType<bool>>()
            .fire();

        match return_value {
            Ok(heads_or_tails) => {
                ink_env::debug_println(&format!("get method call success"));
                ink_env::debug_println(&format!("get return value {}", heads_or_tails));
            },
            Err(e) => {
                ink_env::debug_println(&format!("get method call failed: {:?}", e));
                return Err(Error::LevelContractCallFailed);
            }
        }

        Ok(true)
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
