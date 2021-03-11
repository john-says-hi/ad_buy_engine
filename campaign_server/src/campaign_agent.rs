use std::collections::VecDeque;
use std::io;

use actix::actors::resolver::{Connect, Resolver};
use actix::prelude::*;
use actix_utils::oneshot;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use log::{error, info, warn};

use crate::utils::errors::ApiError;
use futures::FutureExt;

#[derive(Debug)]
pub enum Command {
    CleanLiveCampaigns(()),
    AddClickToRecord(()),
    CleanUpDNS,
    TotalUsersData,
}

pub enum VisitAgentMessage {
    UpdateVisit,
    PostbackDetected,
}

impl Message for Command {
    type Result = Result<(), ApiError>;
}

pub struct CampaignAgent {
    addr: String,
    backoff: ExponentialBackoff,
    cell: Option<()>,
    queue: VecDeque<oneshot::Sender<Result<(), ApiError>>>,
}

impl CampaignAgent {
    pub fn start<S: Into<String>>(addr: S) -> Addr<CampaignAgent> {
        let addr = addr.into();

        let mut backoff = ExponentialBackoff::default();
        backoff.max_elapsed_time = None;

        Supervisor::start(|_| CampaignAgent {
            addr,
            cell: None,
            backoff,
            queue: VecDeque::new(),
        })
    }
}

impl Actor for CampaignAgent {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Agent Started...");
    }
}

impl Supervised for CampaignAgent {
    fn restarting(&mut self, _: &mut Self::Context) {
        self.cell.take();
        for tx in self.queue.drain(..) {
            let _ = tx.send(Err(ApiError::Disconnected));
        }
    }
}

impl Handler<Command> for CampaignAgent {
    type Result = ResponseFuture<Result<(), ApiError>>;

    fn handle(&mut self, msg: Command, _: &mut Self::Context) -> Self::Result {
        let (tx, rx) = oneshot::channel();
        if let Some(ref mut cell) = self.cell {
            self.queue.push_back(tx);
            // cell.write(msg.0);
        } else {
            let _ = tx.send(Err(ApiError::NotConnected));
        }

        Box::pin(rx.map(|res| match res {
            Ok(res) => res,
            Err(_) => Err(ApiError::Disconnected),
        }))
    }
}
