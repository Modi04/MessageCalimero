use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::serde::Serialize;
use calimero_sdk::{app, env};
use std::time::{SystemTime, UNIX_EPOCH};


#[app::state(emits = Event)]
#[derive(BorshDeserialize, BorshSerialize, Default, Debug)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct CalLock {
    pub chats: Vec<Chat>,
    pub messages: Vec<Message>,
}

#[derive(BorshDeserialize, BorshSerialize, Default, Serialize, Debug, Clone)] // Clone 추가
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Chat {
    pub id: String,
    pub context: String,
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Default, Serialize, Debug, Clone)] // Clone 추가
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Message {
    pub id: u64,
    pub chat_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: u64,
}

#[app::event]
pub enum Event {
    ChatCreated {
        id: String,
    },
    MessageCreated {
        id: u64,
    },
}

#[app::logic]
impl CalLock {
    #[app::init]
    pub fn init() -> CalLock {
        CalLock::default()
    }

    pub fn view_user_chats(&self, user_id: &str, context_id: &str) -> Vec<Chat> {
        return self.chats
            .iter()
            .filter(|chat| chat.context == context_id)
            .filter(|chat| chat.id.contains(user_id))
            .cloned() // 소유권 반환
            .collect()
    }

    pub fn view_chat(&self, chat_id: &str) -> Vec<Chat> {
        return self.chats
            .iter()
            .filter(|chat| chat.id == chat_id)
            .cloned() // 소유권 반환
            .collect()
    }

    pub fn create_user_chat(&mut self, id: String, context: String, name: String) {
        let chat = Chat { id : id.clone(), context, name };
        self.chats.push(chat);
        app::emit!(Event::ChatCreated {
            id : id,
        });
    }

    pub fn view_chat_message<'a>(&'a self, chat_id: &str) -> Vec<&'a Message> {
        self.messages
            .iter()
            .filter(|message| message.chat_id == chat_id)
            .collect()
    }

    pub fn create_chat_message(&mut self, chat_id: String, user_id: String, content: String) {
        // 메시지 ID 및 생성 시간은 현재 메시지 수에 기반
        let index = self.messages.len() as u64;
    
        // 메시지 생성
        let message = Message {
            id: index,               // 메시지 ID를 index로 설정
            chat_id: chat_id.clone(), // chat_id 설정
            user_id: user_id.clone(), // user_id 설정
            content: content.clone(), // 메시지 내용 설정
            created_at: index,        // 생성 시간을 index로 설정
        };
        app::emit!(Event::MessageCreated {
            id: index,
        });
        
    
        // 메시지를 메시지 리스트에 추가
        self.messages.push(message);
    }
    
}
