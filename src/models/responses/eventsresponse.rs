use crate::models::event::Event;
use serde::Serialize;

#[derive(Serialize)]
pub struct EventsResponse { 
    pub events: Vec<Event>
}
