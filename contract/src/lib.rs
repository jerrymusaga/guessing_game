
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen};


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    secret_number: u64
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(secret_number: u64) -> Self {
        Self { secret_number }
    }

    pub fn get_secret_number(&self) -> u64{
        self.secret_number.clone()
    }

    pub fn make_guess(&mut self, guess: u64) -> bool {
        // // Use env::log to record logs permanently to the blockchain!
        // log!("Saving guess {}", guess);
        if self.secret_number == guess{
            env::log_str("You got the right guess");
            true
        }
        else {
            env::log_str("Wrong guess");
            false
        }
        
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn get_default_greeting() {
    //     let contract = Contract::default();
    //     // this test did not call set_greeting so should return the default "Hello" greeting
    //     assert_eq!(
    //         contract.get_greeting(),
    //         "Hello".to_string()
    //     );
    // }

    // #[test]
    // fn set_then_get_greeting() {
    //     let mut contract = Contract::default();
    //     contract.set_greeting("howdy".to_string());
    //     assert_eq!(
    //         contract.get_greeting(),
    //         "howdy".to_string()
    //     );
    // }
}
