use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};

use crate::*;

use near_sdk::serde::{Deserialize, Serialize};

// TODO We will need a proper struct for account, for holding additional account data
// like pfp's or whatever
#[derive(Debug, Default, BorshSerialize, BorshDeserialize, PartialEq, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    /*timestamp: TimeStamp,*/
    /*attached_content: File,*/
    receiver: Option<String>,
    sender: String,
    content: String,
}

#[derive(Debug, PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MessageWithLen {
    pub len: u64,
    pub content: Vec<Message>,
}

impl MessageWithLen {
    pub fn new(len: u64, content: Vec<Message>) -> Self {
        MessageWithLen {
            len: len,
            content: content,
        }
    }
}

impl Message {
    pub fn new(receiver: Option<String>, sender: String, message: String) -> Self {
        Self {
            receiver: receiver,
            sender: sender,
            content: message,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn send_message(&mut self, receiver: Option<String>, message: String) {
        let sender = env::predecessor_account_id();

        self.messages
            .push(&Message::new(receiver, sender, message.to_string()));
    }

    #[private]
    pub fn load_messages(&self, amount: u64) -> Option<MessageWithLen> {
        match self.messages.is_empty() {
            true => None,
            _ => {
                let mut count = 1;
                let len = self.messages.len();
                let default = Message::default();

                let mut messages: Vec<Message> = Vec::new();

                // Check for default is accompanied "&& count < len"(len is
                // the name of the varible which has value "self.messages.len();")
                while count < amount && count == len {
                    let message = self.messages.get(len - count).unwrap_or_default();
                    if message == default {
                        break;
                    }
                    messages.push(message);
                    println!("{}", count);
                    count += 1;
                }
                Some(MessageWithLen::new(len, messages))
            }
        }
    }

    pub fn get_messages(&self) -> Option<MessageWithLen> {
        self.load_messages(50)
    }

    // listen for any new messages since the last was sent from the smart contract
    pub fn listen(&self, old_len: u64) -> Option<MessageWithLen> {
        let current_len = self.messages.len();
        let len = current_len - old_len;

        if len > 0 {
            if len > 50 {
                self.load_messages(50)
            } else {
                self.load_messages(len)
            }
        } else {
            None
        }
    }
}

//MessageWithLen::new(self.messages.len(), get_messages())
