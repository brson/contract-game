#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod game {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap;
    
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
    pub enum Error {
	InsufficiantBalance,	
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
	pub fn create_game_account(&mut self) -> Result<GameAccount, Error> {
            let caller = self.env().caller();
	    let balance = self.env().transferred_balance();

	    // create a captain costs 1000 money?
	    // todo: return transferred_balance on Error?
	    if balance < 1000 {
		ink_env::debug_println("Your balance isn't enough for creating your captain");
		Err(Error::InsufficiantBalance)
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

    // Methos support contracts
    impl Game {
	fn create_a_captain(&mut self, account: AccountId) -> Result<GameAccount, Error> {
	    // return an error if account/address is exists in the hashmap
	    // create new captain_account
	    // insert into the hashmap
	    
	    panic!()
	}
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;

    }
}

