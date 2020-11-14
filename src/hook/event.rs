extern crate serde;

use serde::{Serialize, Serializer};

#[derive(PartialEq, Copy, Clone)]
pub enum HookEvent {
    Login,
    Signup,
    Update,
}

impl Serialize for HookEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        if *self == HookEvent::Login {
            return serializer.serialize_str("login");
        } else if *self == HookEvent::Update {
            return serializer.serialize_str("update");
        }

        return serializer.serialize_str("signup");
    }
}
