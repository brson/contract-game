#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod game {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap;
    use alloc::{string::String, format};

    
    #[ink(storage)]
    pub struct Game {
	game_accounts: HashMap<AccountId, GameAccount>,
    }

    #[derive(Debug, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct GameAccount {
	game_account_id: [u32; 8],
	level: u32,
	// todo: other data for a captain
	// e.g.: NFT pet, Erc20 gold in game
    }

    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct Error; 
    
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
	    Game {
		game_accounts: HashMap::new(),
	    }
        }

	/// Query if the caller has an account
	#[ink(message)]
	pub fn have_game_account(&self) -> bool {
	    panic!()
	}

	/// Create an account for the caller
	///
	/// # Errors
	///
	/// - The account exists.
	/// - The paid amount is insufficient.
	#[ink(message, payable)]
	pub fn create_game_account(&mut self, role_name: String) -> Result<GameAccount, Error> {
            let caller = self.env().caller();
	    let balance = self.env().transferred_balance();

	    if role_name_is_valid() { 
		if balance < 1000 {
		    ink_env::debug_println(&format!("Your balance isn't enough for creating your captain"));
		    panic!()
		} else {
		    // create a captain costs xxxx money?
		    create_a_captain(caller, &role_name)
		}
	    } else {
		panic!()
	    }		
	}

	/// Retrieve caller's account information
	///
	/// # Errors
	///
	/// - The account doesn't exist.
	#[ink(message)]
	pub fn get_game_account(&self) -> Result<GameAccount, Error> {
	    panic!()
	}

	/// Submit a program for a level puzzle
	///
	/// # Errors
	///
	/// - Level is greater than caller's current level.
	/// - Program fails verification.
	/// - Program account doesn't exist.
	#[ink(message, payable)]
	pub fn submit_level(&mut self, level: u32, program_id: AccountId) -> Result<(), Error> {
	    panic!()
	}

	/// Run a level
	///
	/// # Errors
	///
	/// - Level is greater than caller's current level.
	/// - Caller has no submiss for this level.
	/// - Submitted program doesn't implement required contracts.
	#[ink(message, payable)]
	pub fn run_level(&mut self, level: u32) -> Result<(), Error> {
	    panic!()
	}
    }

    fn role_name_is_valid() -> bool { 
	panic!()
    }

    fn create_a_captain(account: AccountId, role_name: &str) -> Result<GameAccount, Error> {
	panic!()
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;

    }
}

