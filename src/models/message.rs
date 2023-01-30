use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..30))]
    room: String,

    #[field(validate = len(..20))]
    username: String,

    message: String
}