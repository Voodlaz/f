// Some comments could probably fit in implementation doc

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

// This struct is needed for fn listen in impl contract in the end of file.
#[derive(Debug, PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MessageWithLen {
    pub len: u64,
    pub content: Vec<Message>,
}

// used for Contract::lsiten, implemented in the end of file
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
    pub fn load_messages(&self, amount: u64, levels: u64) -> Option<MessageWithLen> {
        match self.messages.is_empty() {
            true => None,
            _ => {
                let mut count = 1;
                let len = self.messages.len();
                let mut len_minus_levels;

                match levels {
                    0 => len_minus_levels = len,
                    // unwrap_or_default maybe changed to just unwrap in the future. see
                    // the if block comments below for more.
                    _ => len_minus_levels = len.checked_sub(levels * 50).unwrap_or_default(),
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
                if len_minus_levels == 0 {
                    len_minus_levels = len - len % 50
                }

                let mut messages: Vec<Message> = Vec::new();

                let default = Message::default();
                while count < amount + 1 && count < len_minus_levels + 1 {
                    let message = self
                        .messages
                        .get(len_minus_levels - count)
                        .unwrap_or_default();
                    if message == default {
                        break;
                    }
                    messages.push(message);
                    count += 1;
                }
                println!("{:?}", messages.len());
                Some(MessageWithLen::new(len, messages))
            }
        }
    }
    /*we load 50 messages in advance, so when a person is scrolling
    through the cronological line, and gets close to the limit
    of already loaded messages on client, more 50 messages will
    load(WIP), so he would have a seamless reading experience.
    NOTE: this decison is not final, and can change in future*/
    pub fn get_messages(&self, levels: u64) -> Option<MessageWithLen> {
        self.load_messages(50, levels)
    }

    /*listen for any new messages since the last was sent from the smart contract.
    Uses MessageWithLen to send len of messages Vector with it's contents, so
    the client could send back the len when it wants to check for new messages.
    If the Vector is bigger than the one sent by client, it means that new
    messages were sent, and current len minus len sent by client equals the
    amount of new messages, which the functions sends to client.*/
    pub fn listen(&self, old_len: u64) -> Option<MessageWithLen> {
        let current_len = self.messages.len();
        let len = current_len - old_len;

        if len > 0 {
            if len > 50 {
                self.load_messages(50, 0)
            } else {
                self.load_messages(len, 0)
            }
        } else {
            None
        }
    }
}

//MessageWithLen::new(self.messages.len(), get_messages())
