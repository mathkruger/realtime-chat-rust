use rocket::{
    form::Form,
    State,
    tokio::sync::broadcast::Sender
};

use crate::models::message::Message;

#[post("/message", data = "<form>")]
pub fn send_message(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}