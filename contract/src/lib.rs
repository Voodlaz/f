mod message;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault};

use crate::message::*;
use near_sdk::serde::Serialize;

setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
//#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    pub messages: Vector<message::Message>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        //let members = UnorderedSet::new(b"r");
        let messages = Vector::new(b"m");

        Self {
            //members: members,
            messages: messages,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob.near".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn plain_text() {
        let context = get_context(false);
        testing_env!(context.clone());

        let mut contract = Contract::new();
        let sender = context.predecessor_account_id;

        contract.send_message(None, "first".to_string());
        contract.send_message(None, "second".to_string());
        contract.send_message(None, "third".to_string());

        let mut vecr = Vec::new();

        vecr.push(Message::new(None, sender.clone(), "third".to_string()));
        vecr.push(Message::new(None, sender.clone(), "second".to_string()));
        vecr.push(Message::new(None, sender.clone(), "first".to_string()));

        // get_message tests
        assert_eq!(
            MessageWithLen::new(3, vecr).content,
            contract.get_messages().unwrap().content
        );
        //listen test
        /*assert_eq!(
            MessageWithLen::new(1, vecr).content,
            contract.listen(1).unwrap().content
        );*/
        // len test
        /*assert_eq!(
            MessageWithLen::new(3, vecr).len,
            contract.get_messages().unwrap().len
        )*/
    }
}
