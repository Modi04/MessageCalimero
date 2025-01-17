use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::serde::Serialize;
use calimero_sdk::{app, env};

#[app::state(emits = for<'a> Event<'a>)]
#[derive(BorshDeserialize, BorshSerialize, Default)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct Research {
    posts: Vec<Post>,
}

#[derive(BorshDeserialize, BorshSerialize, Default, Serialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Post {
    id: usize,
    content: String,
}

#[app::event]
pub enum Event<'a> {
    PostCreated {
        id: usize,
        content: &'a str,
    },
}

#[app::logic]
impl Research {
    #[app::init]
    pub fn init() -> Research {
        Research::default()
    }

    pub fn post(&self, id: usize) -> Option<&Post> {
        env::log(&format!("Getting post with id: {:?}", id));

        self.posts.get(id)
    }

    pub fn posts(&self) -> &[Post] {
        env::log("Getting all posts");

        &self.posts
    }

    pub fn create_post(&mut self, content: String) -> &Post {
        env::log(&format!(
            "Creating post with content: {:?}",
            content
        ));

        app::emit!(Event::PostCreated {
            id: self.posts.len(),
            // todo! should we maybe only emit an ID, and let notified clients fetch the post?
            content: &content,
        });

        self.posts.push(Post {
            id: self.posts.len(),
            content,
        });

        match self.posts.last() {
            Some(post) => post,
            None => env::unreachable(),
        }
    }
}
