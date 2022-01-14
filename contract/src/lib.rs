use near_sdk::{near_bindgen, PanicOnDefault, setup_alloc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;

mod message;

//use rand::random;
//use near_sdk::serde::Serialize;

setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
//#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    messages: Vector<message::Message>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        //let members = UnorderedSet::new(b"r");
        let messages = Vector::new(b"m");

        Self {
            //members: members,
            messages,
        }
    }
}
