#[macro_use] extern crate rocket;

mod models;
mod routes;

use rocket::{
    tokio::sync::broadcast::channel,
    fs::{
        relative,
        FileServer
    }
};

use models::message::Message;
use routes::post::send_message;
use routes::get::events;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(channel::<Message>(1024).0)
    .mount("/", routes![
        send_message,
        events
    ])
    .mount("/", FileServer::from(relative!("static")))
}
