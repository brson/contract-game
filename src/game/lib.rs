#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod game {
    #[ink(storage)]
    pub struct Game {

    }

    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] // todo: what is this?
    pub struct Account {
	level: u32,
    }
    
    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
	    Game {}
        }

	/// Query if the caller has an account
	#[ink(message)]
	pub fn have_account(&self) -> bool {
	    panic!()
	}

	/// Create an account for the caller
	///
	/// # Errors
	///
	/// - The account exists.
	/// - The paid amount is insufficient.
	#[ink(message, payable)]
	pub fn create_account(&mut self) -> Result<(), ()> {
	    panic!()
	}

	/// Retrieve caller's account information
	///
	/// # Errors
	///
	/// - The account doesn't exist.
	#[ink(message)]
	pub fn get_account(&self) -> Result<Account, ()> {
	    panic!()
	}
    }

    #[cfg(test)]
    mod tests {
        use super::*;

    }
}
