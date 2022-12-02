use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    ///A unique id for each todo
    pub id: String,

    /// todo
    pub text: String,

    ///A unique id for each todo
    pub done: bool,
}
