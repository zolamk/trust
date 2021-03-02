extern crate serde;

use serde::{Serialize, Serializer};

#[derive(PartialEq, Copy, Clone)]
pub enum HookEvent {
    Login,
    Update,
}

impl Serialize for HookEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        if *self == HookEvent::Login {
            return serializer.serialize_str("login");
        }

        return serializer.serialize_str("update");
    }
}
