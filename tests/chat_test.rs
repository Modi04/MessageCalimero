use cal_lock::{CalLock, Chat, Message};
use chrono::Local;

fn create_new_cal_lock() -> CalLock {
    CalLock {
        chats: Vec::new(),
        messages: Vec::new(),
    }
}

#[test]
fn test_view_user_chats_found() {
    let db = CalLock {
        chats: vec![
            Chat {
                id: "chat1_user1".to_string(),
                context: "context1".to_string(),
                name: "General Chat".to_string(),
            },
            Chat {
                id: "chat2_user2".to_string(),
                context: "context1".to_string(),
                name: "Private Chat".to_string(),
            },
        ],
        messages: Vec::new(),
    };

    let result = db.view_user_chats("user1", "context1");
    println!("Result: {:?}", result);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "chat1_user1");
}

#[test]
fn test_create_user_chat() {
    let mut db = create_new_cal_lock();
    db.create_user_chat("chat1".to_string(), "context1".to_string(), "General Chat".to_string());

    assert_eq!(db.chats.len(), 1);
    assert_eq!(db.chats[0].id, "chat1");
    assert_eq!(db.chats[0].context, "context1");
    assert_eq!(db.chats[0].name, "General Chat");
}

#[test]
fn test_view_chat_message() {
    let mut db = create_new_cal_lock();
    db.create_chat_message("chat1".to_string(), "user1".to_string(), "Hello, world!".to_string());

    let messages = db.view_chat_message("chat1");
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello, world!");
}

#[test]
fn test_create_chat_message_with_current_time() {
    let mut db = create_new_cal_lock();
    db.create_chat_message("chat1".to_string(), "user1".to_string(), "Hello, world!".to_string());

    let message = &db.messages[0];
    println!("Created At: {}", message.created_at);

    assert!(message.created_at.len() > 0); // created_at 값이 존재하는지 확인
}
