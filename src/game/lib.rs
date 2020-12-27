#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod game {
    #[ink(storage)]
    pub struct Game {

    }

    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
	    Game {}
        }

        #[ink(message)]
        pub fn flip(&mut self) {
	    panic!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

    }
}
