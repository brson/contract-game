#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod game {
    #[ink(storage)]
    pub struct Game {

    }

    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct GameAccount {
	level: u32,
    }

    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct Error; 
    
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
	    Game {}
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
	pub fn create_game_account(&mut self) -> Result<(), Error> {
	    panic!()
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

    #[cfg(test)]
    mod tests {
        use super::*;

    }
}
