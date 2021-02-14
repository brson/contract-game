#![allow(unused)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod game {
    use alloc::collections::BTreeMap;
    use alloc::format;
    use alloc::string::{String, ToString};
    use ink_env::call::build_call;
    use ink_env::call::utils::ReturnType;
    use ink_env::call::ExecutionInput;
    use ink_env::call::Selector;
    use ink_env::DefaultEnvironment;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct Game {
        player_accounts: HashMap<AccountId, PlayerAccount>,
    }

    #[derive(
        Debug,
        Clone,
        scale::Encode,
        scale::Decode,
        ink_storage_derive::PackedLayout,
        ink_storage_derive::SpreadLayout,
    )]
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

        fn level_up(&mut self) {
            self.level += 1;
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
        LevelContractNotPass,
    }

    // Contract methods
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game {
                player_accounts: HashMap::new(),
            }
        }

        /// This is a sanity check for the application initialization code
        #[ink(message)]
        pub fn game_ready(&self) -> String {
            "heck, yeah".to_string()
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
                self.create_player(caller)
            }
        }

        /// Retrieve caller's account information
        ///
        /// # Errors
        ///
        /// - The account doesn't exist.
        #[ink(message)]
        pub fn get_player_account(&self, account: AccountId) -> Result<PlayerAccount, Error> {
            ink_env::debug_println(&format!(
                "Get player account {:?}",
                self.player_accounts.get(&account).cloned()
            ));
            self.player_accounts
                .get(&account)
                .cloned()
                .ok_or(Error::AccountNotExists)
        }

        /// Submit a program for a level puzzle
        ///
        /// # Errors
        ///
        /// - Level is greater than caller's current level.
        /// - Program fails verification.
        /// - Program account doesn't exist.
        #[ink(message)]
        pub fn submit_level(&mut self, level: u32, level_contract: AccountId) -> Result<(), Error> {
            let caller = self.env().caller();

            if let Some(player_account) = self.player_accounts.get_mut(&caller) {
                let account_current_level = player_account.level;
                if level > account_current_level {
                    ink_env::debug_println(&format!("Submitted Greater Level"));
                    Err(Error::SubmittedGreaterLevel)
                } else {
                    ink_env::debug_println(&format!(
                        "insert level {}, and contract {:?}",
                        level, level_contract
                    ));
                    player_account.level_contracts.insert(level, level_contract);
                    Ok(())
                }
            } else {
                ink_env::debug_println(&format!("Account Not Exists"));
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
        pub fn run_level(&mut self, level: u32) -> Result<(), Error> {
            let caller = self.env().caller();

            if let Some(player_account) = self.player_accounts.get_mut(&caller) {
                ink_env::debug_println(&format!("game account: {:?}", player_account));

                if let Some(level_contract) = player_account.level_contracts.get(&level).cloned() {
                    ink_env::debug_println(&format!("program id: {:?}", level_contract));

                    let result = dispatch_level(level, level_contract);
                    match result {
                        Ok(_) => {
                            // only update level when
                            // - the program level is equal to the player's current (highest) level
                            // - current level isn't the Game's highest level (set as 2)
                            if level == player_account.level && level != 2 {
                                player_account.level_up();
                                ink_env::debug_println(&format!(
                                    "updated_player_account: {:?}",
                                    &player_account
                                ));
                            }
                            return Ok(());
                        }
                        Err(e) => {
                            ink_env::debug_println(&format!("dispatch_level failed: {:?}", e));
                            return Err(e);
                        }
                    }
                } else {
                    ink_env::debug_println(&format!("level_contract doesn't exist: {:?}", caller));
                    Err(Error::LevelContractNotExists)
                }
            } else {
                ink_env::debug_println(&format!("player account doesn't exist: {:?}", caller));
                Err(Error::AccountNotExists)
            }
        }
    }

    // Non-contract support methods
    impl Game {
        fn create_player(&mut self, account: AccountId) -> Result<PlayerAccount, Error> {
            let new_player_account = PlayerAccount::default();
            self.player_accounts
                .insert(account, new_player_account.clone());
            ink_env::debug_println(&format!("new player account {:?}", new_player_account));

            Ok(new_player_account)
        }
    }

    fn dispatch_level(level: u32, level_contract: AccountId) -> Result<(), Error> {
        ink_env::debug_println(&format!(
            "dispatch level: {}, calling contract: {:?}",
            level, level_contract
        ));

        // Game's highest level is 2
        match level {
            0 => run_level_0_flipper(level_contract),
            1 => run_level_1_flipper(level_contract),
            2 => run_level_2_flipper(level_contract),
            _ => return unreachable!(),
        }
    }

    fn run_level_0_flipper(level_contract: AccountId) -> Result<(), Error> {
        ink_env::debug_println(&format!(
            "run_level_0_flipper, calling contract: {:?}",
            level_contract
        ));

        let flipper_current_state = build_call::<DefaultEnvironment>()
            .callee(level_contract)
            .exec_input(ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF])))
            .returns::<ReturnType<bool>>()
            .fire();

        let flipper_current_state = match flipper_current_state {
            Err(e) => {
                ink_env::debug_println(&format!("flipper_current state failed: {:?}", e));
                return Err(Error::LevelContractCallFailed);
            }
            Ok(f) => f,
        };

        ink_env::debug_println(&format!("verified flipper current state"));

        let flipper_set_state = build_call::<DefaultEnvironment>()
            .callee(level_contract)
            .exec_input(ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF])))
            .returns::<ReturnType<()>>()
            .fire();

        if let Err(e) = flipper_set_state {
            ink_env::debug_println(&format!("flipper_current state failed: {:?}", e));
            return Err(Error::LevelContractCallFailed);
        }

        let flipper_new_state = build_call::<DefaultEnvironment>()
            .callee(level_contract)
            .exec_input(ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF])))
            .returns::<ReturnType<bool>>()
            .fire();

        let flipper_new_state = match flipper_new_state {
            Err(e) => {
                ink_env::debug_println(&format!("flipper_new_state failed"));
                return Err(Error::LevelContractCallFailed);
            }
            Ok(f) => f,
        };

        ink_env::debug_println(&format!("verify flipper new state"));

        if flipper_current_state == flipper_new_state {
            ink_env::debug_println(&format!("verify flipper_current_state failed"));
            return Err(Error::LevelContractCallFailed);
        }

        ink_env::debug_println(&format!("run_level_0_flipper call success"));

        Ok(())
    }

    fn run_level_1_flipper(level_contract: AccountId) -> Result<(), Error> {
        ink_env::debug_println(&format!(
            "run_level_1_flipper, calling contract: {:?}",
            level_contract
        ));

        run_level_0_flipper(level_contract)
    }

    fn run_level_2_flipper(level_contract: AccountId) -> Result<(), Error> {
        ink_env::debug_println(&format!(
            "run_level_2_flipper, calling contract: {:?}",
            level_contract
        ));

        let result = run_level_0_flipper(level_contract);
        match result {
            Err(e) => Err(e),
            Ok(_) => {
                ink_env::debug_println(&format!(
                    "Congratulations, Captain! You have passed all the levels!"
                ));
                Ok(())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
