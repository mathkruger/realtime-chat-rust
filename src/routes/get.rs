use rocket::{
    State,
    tokio::{
        select,
        sync::broadcast::{
            Sender,
            error::RecvError
        }
    },
    Shutdown,
    response::stream::{
        EventStream,
        Event
    }
};

use crate::models::message::Message;

#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}