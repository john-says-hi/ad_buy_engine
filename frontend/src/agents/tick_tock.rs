use crate::notify_primary;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum TickTockResponse {
    Tock,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TickTockRequest {
    Tick,
}

pub struct TickTock {
    link: AgentLink<TickTock>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for TickTock {
    type Reach = Context<Self>;
    type Message = ();
    type Input = TickTockRequest;
    type Output = TickTockResponse;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            TickTockRequest::Tick => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, TickTockResponse::Tock);
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
