use near_sdk::{env, near_bindgen};
// TODO this file should be broken into smaller modules
// TODO refactor comments
// Some comments could probably fit in implementation doc
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic;
use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

//use std::iter::IntoIterator;

#[derive(
    Debug, Default, BorshSerialize, BorshDeserialize, PartialEq, Serialize, Deserialize, Clone,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    /*timestamp: TimeStamp,*/
    /*attached_content: File,*/
    receiver: Option<String>,
    sender: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ErrorMessage {
    NoNewMessages,
    IncorrectData,
}

// This struct is needed for fn listen in impl contract in the end of file.
#[derive(Debug, PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MessageWithLen(u64, Vec<Message>);

impl Message {
    pub fn new(receiver: Option<String>, sender: String, message: String) -> Self {
        Self {
            receiver,
            sender,
            content: message,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn purge(&mut self, password: String) {
        match &password as &str {
            "7ypn6~]42h5;G^=J" => self.messages.clear(),
            _ => panic!("go away"),
        }
    }

    pub fn send_message(&mut self, receiver: Option<String>, message: String) {
        let sender = env::predecessor_account_id();

        self.messages.push(&Message::new(receiver, sender, message));
    }

    #[private]
    fn load_messages(&self, amount: u64, levels: u64) -> Result<MessageWithLen, ErrorMessage> {
        match self.messages.is_empty() {
            true => Err(ErrorMessage::NoNewMessages),
            false => {
                let mut count = 1;
                let len = self.messages.len();
                let mut len_minus_levels;

                match levels {
                    0 => len_minus_levels = len,
                    // unwrap_or_default maybe changed to just unwrap in the future. see
                    // the if block comments below for more.
                    _ => {
                        len_minus_levels = {
                            match len.checked_sub(levels * 50) {
                                Some(x) => x,
                                None => len % 50,
                            }
                        }
                    }
                }
                /*this if code block moves the len "cursor"(the strating point
                of reading messages) to the maximum level possible.
                This can be removed in future, as fronted is implemented
                because how fronted jumps to levels is not yet decided(and
                will be decided with the implementation). Also see doc 38
                about that new ideas on that topic.

                unwrap_or_default will be useless if if block get's removed

                u64::default equals to zero, so unwrap_or_default works*/
                // what about making the fronted able to request max levels?

                let mut messages: Vec<Message> = Vec::new();

                while count < amount + 1 && count < len_minus_levels + 1 {
                    let message = self.messages.get(len_minus_levels - count);
                    match message {
                        Ok(x) => {
                            messages.push(x);
                            count += 1;
                        }
                        None => break,
                    }
                }
                Ok(MessageWithLen(len, messages))
            }
        }
    }
    /*we load 50 messages in advance, so when a person is scrolling
    through the cronological line, and gets close to the limit
    of already loaded messages on client, more 50 messages will
    load(WIP), so he would have a seamless reading experience.
    NOTE: this decison is not final, and can change in future*/
    pub fn get_messages(&self, levels: u64) -> Result<MessageWithLen, ErrorMessage> {
        self.load_messages(50, levels)
    }

    /*listen for any new messages since the last was sent from the smart contract.
    Uses MessageWithLen to send len of messages Vector with it's contents, so
    the client could send back the len when it wants to check for new messages.
    If the Vector is bigger than the one sent by client, it means that new
    messages were sent, and current len minus len sent by client equals the
    amount of new messages, which the functions sends to client.*/
    //
    /* should we load only the last ones, or just messages after old_len?*/
    pub fn listen(&self, old_len: u64) -> Result<MessageWithLen, ErrorMessage> {
        let len = self.messages.len().checked_sub(old_len);
        let load_messages =
            |amount| -> Result<MessageWithLen, ErrorMessage> { self.load_messages(amount, 0) };

        match len {
            Some(x) => match x {
                0 => Err(ErrorMessage::NoNewMessages),
                _ => {
                    if x > 50 {
                        load_messages(50)
                    } else {
                        load_messages(x)
                    }
                }
            },
            None => Err(ErrorMessage::IncorrectData),
        }
    }
}

//MessageWithLen::new(self.messages.len(), get_messages())

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::convert::TryInto;

    use near_sdk::{MockedBlockchain, testing_env, VMContext};
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob.near".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    // makes a Vec of messages, sends them to contract
    // and gives back the list, so it could be possible
    // to test out view methods
    fn view_methods(contract: &mut Contract, sender: String) -> Vec<Message> {
        let mut vecr: Vec<Message> = Vec::new();
        //let mut iterator =

        for i in 0..57 {
            vecr.push(Message::new(None, sender.clone(), i.to_string()))
        }

        for i in vecr.iter() {
            contract.send_message(None, i.content.clone())
        }

        vecr
    }

    #[test]
    fn purge() {
        let context = get_context(false);
        testing_env!(context.clone());
        let mut contract = Contract::new();

        let vecr = view_methods(&mut contract, context.predecessor_account_id);

        contract.purge("7ypn6~]42h5;G^=J".to_string());

        match contract.get_messages(0) {
            Ok(x) => panic!("it doesn't work. here's what actually there {:?}", x.1),
            Err(e) => (),
        }
    }

    #[test]
    fn get_messages() {
        let context = get_context(false);
        testing_env!(context.clone());
        let mut contract = Contract::new();

        let vecr = view_methods(&mut contract, context.predecessor_account_id);

        assert_eq!(
            {
                let mut result: Vec<Message> = Vec::new();
                for i in &vecr[vecr.len() - 50..] {
                    result.insert(0, i.clone())
                }
                result
            },
            contract.get_messages(0).unwrap().1
        );
    }

    #[test]
    fn listen() {
        let context = get_context(false);
        testing_env!(context.clone());
        let mut contract = Contract::new();

        let vecr = view_methods(&mut contract, context.predecessor_account_id);

        assert_eq!(
            {
                let mut result: Vec<Message> = Vec::new();
                for i in &vecr[24..] {
                    result.insert(0, i.clone())
                }
                result
            },
            contract.listen(24).unwrap().1
        );
    }
}
