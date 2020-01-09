use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ref(String);

impl Ref {
    pub fn from_string(s: String) -> Self {
        Ref(s)
    }

    pub fn from_str(s: &str) -> Self {
        Ref::from_string(s.to_string())
    }
}
