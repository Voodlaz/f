/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. send_message: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub messages: UnorderedMap<String, String /*Message*/>,
}

/*impl Default for Contract {
    fn default() -> Self {
        let messages = UnorderedMap::new(b"m");
        Self { messages: messages }
    }
}*/

#[near_bindgen]
impl Contract {
    #[init]
    fn new() -> Self {
        //let members = UnorderedSet::new(b"r");
        let messages = UnorderedMap::new(b"m");

        Self {
            //members: members,
            messages: messages,
        }
    }

    pub fn send_message(&mut self, message: String) {
        let sender = env::predecessor_account_id();
        self.messages.insert(
            &sender.to_string(),
            &message.to_string(), /*&Message::new(message.to_string())*/
        );
    }

    pub fn get_greeting(&self, account_id: String) -> String {
        self.messages.get(&account_id).unwrap()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
