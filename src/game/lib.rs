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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

    }
}
