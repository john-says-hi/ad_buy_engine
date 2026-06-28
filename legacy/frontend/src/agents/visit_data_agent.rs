use crate::prelude::*;
use yew::worker::*;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SyncAction {
    GetAppState,
    SyncClickData,
    SyncCampaignElement(EitherCampaignElement),
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    Sync(SyncAction),
}

pub struct SyncAgent {
    pub link: AgentLink<SyncAgent>,
    pub app_state: Rc<AppState>,
}

impl Agent for SyncAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Rc<AppState>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            app_state: Rc::new(AppState::init()),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, hid: HandlerId) {
        match msg {
            Request::Sync(action) => match action {
                SyncAction::SyncCampaignElement(either) => match either.campaign_element {
                    Either::Left(element) => match element {
                        CampaignElement::OfferSource(mirror) => {
                            if let Some(pos) = self
                                .app_state
                                .offer_sources
                                .borrow()
                                .iter()
                                .position(|s| s.id == mirror.id)
                            {
                                let mut handle = self.app_state.offer_sources.borrow_mut();
                                handle.remove(pos);
                                handle.insert(pos, mirror);
                            } else {
                                self.app_state.offer_sources.borrow_mut().push(mirror)
                            }
                        }
                        _ => {}
                    },
                    Either::Right(elements) => match elements {
                        CampaignElements::OfferSources(mirrors) => {
                            for mirror in mirrors {
                                if let Some(pos) = self
                                    .app_state
                                    .offer_sources
                                    .borrow()
                                    .iter()
                                    .position(|s| s.id == mirror.id)
                                {
                                    let mut handle = self.app_state.offer_sources.borrow_mut();
                                    handle.remove(pos);
                                    handle.insert(pos, mirror);
                                } else {
                                    self.app_state.offer_sources.borrow_mut().push(mirror)
                                }
                            }
                        }
                        _ => {}
                    },
                },
                SyncAction::GetAppState => self.link.respond(hid, Rc::clone(&self.app_state)),
                _ => {}
            },
        }
    }
}
