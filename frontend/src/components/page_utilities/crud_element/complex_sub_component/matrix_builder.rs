use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_sequence_builder::{RHSSequenceBuilder, Msg as SeqMsg};
use crate::components::page_utilities::crud_element::dropdowns::landing_page_dropdown::LandingPageDropdown;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::constant::{
    COLOR_BLUE, DEPTH_0, DEPTH_1, DEPTH_2, DEPTH_3, DEPTH_4, DEPTH_5, DEPTH_6, DEPTH_7, DEPTH_8,
    DEPTH_9,
};
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::matrix::{Matrix, MatrixData, Transform};
use ad_buy_engine::data::elements::offer::Offer;
use serde::de::Unexpected::Seq;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use web_sys::Element;
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_services::storage::Area;
use yew_services::StorageService;
use uuid::Uuid;
use crate::components::page_utilities::update_element::Msg::Update;

pub type RootMatrix = Rc<RefCell<Matrix>>;

pub enum Msg {
    UpdateMatrix(UpdateMatrix),
    UpdateWeight(InputData),
    Ignore,
}

pub enum UpdateMatrix {
    Weight,
    FillVoid(Transform),
    Remove,
    Add(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub root_matrix: Arc<RwLock<Matrix>>,
    pub local_matrix: Arc<RwLock<Matrix>>,
    pub state: STATE,
    pub seq_type: SequenceType,
    pub sequence_builder_link: Rc<Scope<RHSSequenceBuilder>>,
}

pub struct MatrixBuilder {
    link: ComponentLink<Self>,
    props: Props,
    children_hidden: bool,
    weight_buff: u8,
}

impl Component for MatrixBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let wb = match &props.local_matrix.read().expect("G%T$sfdg").value.data {
            MatrixData::Offer(o) => o.weight,
            MatrixData::LandingPage(lp) => lp.weight,
            _ => 100,
        };
        Self {
            children_hidden: true,
            link,
            props,
            weight_buff: wb,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMatrix(update) => {
                match update {
                    UpdateMatrix::FillVoid(trans) => match trans {
                        Transform::Offer(offer) => {
                            self.props.local_matrix.write().expect("G53greg").value.data =
                                MatrixData::Offer(offer);
                        }
                        Transform::Lander(lp) => {
                            self.props.local_matrix.write().expect("G53greg").value.data =
                                MatrixData::LandingPage(lp);
                        }
                    },

                    UpdateMatrix::Weight => {
                        let mut matrix = self.props.local_matrix.write().expect("56gfd");

                        match matrix.value.data.clone() {
                            MatrixData::LandingPage(mut lp) => {
                                lp.weight = self.weight_buff;
                                matrix.value.data = MatrixData::LandingPage(lp);
                            }
                            MatrixData::Offer(mut offer) => {
                                offer.weight = self.weight_buff;
                                matrix.value.data = MatrixData::Offer(offer);
                            }
                            _ => {}
                        }
                    }

                    UpdateMatrix::Remove => {
                        let target_item_id = self
                            .props
                            .local_matrix
                            .read()
                            .expect("g5rtsfdgF")
                            .value
                            .id
                            .clone();

                        let target_group_idx = self
                            .props
                            .local_matrix
                            .read()
                            .expect("g5rtsfdgF")
                            .group_idx();

                        if let Some(parent) = &self
                            .props
                            .local_matrix
                            .read()
                            .expect("g5rtsfdgF")
                            .value
                            .parent_matrix
                        {
                            let parent_node = arc!(parent);

                            if let Some(group) = parent_node
                                .write()
                                .expect("%Gdff")
                                .children_groups
                                .get_mut(target_group_idx)
                            {
                                group.retain(|s| {
                                    s.read().expect("%Gdsfg").value.id.clone() != target_item_id
                                });
                            } else {
                                notify_danger("Group not found")
                            };
                        } else {
                            notify_danger("no parent");
                        }
                    }

                    UpdateMatrix::Add(group_idx) => {
                        let parent_node = arc!(self.props.local_matrix);
                        let mut local_matrix_handle =
                            self.props.local_matrix.write().expect("%^fd");
                        let dept = local_matrix_handle.value.depth;

                        if let Some(group) = local_matrix_handle.children_groups.get_mut(group_idx)
                        {
                            group.insert(
                                group_idx,
                                Arc::new(RwLock::new(Matrix::void(
                                    Some(parent_node),
                                    group_idx,
                                    group.len(),
                                    dept + 1,
                                ))),
                            )
                        }
                    }
                }

                self.props
                    .sequence_builder_link
                    .send_message(SeqMsg::UpdateRootMatrix(arc!(self.props.root_matrix)));
                return true;
            }

            Msg::UpdateWeight(i) => {
                if let Ok(num) = i.value.parse::<u8>() {
                    self.weight_buff = num;
                } else {
                    notify_danger("Invalid")
                }
            }

            // Msg::UpdateMatrix(instruct) => {
            //     match instruct {
            //         UpdateMatrix::Weight(target, value) => match self.props.seq_type {
            //             SequenceType::OffersOnly => {
            //                 self.props
            //                     .root_matrix
            //                     .borrow_mut()
            //                     .children_groups
            //                     .get_mut(0)
            //                     .map(|s| {
            //                         s.iter_mut().find(|s| s.id() == target.id()).map(|s| {
            //                             if let MatrixData::Offer(mut offer) = &s.value.data {
            //                                 offer.weight = value;
            //                             }
            //                         })
            //                     });
            //             }
            //
            //             SequenceType::LandingPageAndOffers => {
            //                 self.props
            //                     .root_matrix
            //                     .borrow_mut()
            //                     .children_groups
            //                     .get_mut(0)
            //                     .map(|s| {
            //                         s.iter_mut().find(|s| s.id() == target.id()).map(|s| {
            //                             if let MatrixData::Offer(mut offer) = &s.value.data {
            //                                 offer.weight = value;
            //                             } else if let MatrixData::LandingPage(mut lp) =
            //                                 &s.value.data
            //                             {
            //                                 lp.weight = value;
            //                             }
            //                         })
            //                     });
            //             }
            //
            //             SequenceType::Matrix => {
            //                 if let Some(parent_node) = target.get_parent_node() {
            //                     let found_parent_res = Matrix::search_next_depth(
            //                         self.props
            //                             .root_matrix
            //                             .borrow_mut()
            //                             .children_groups
            //                             .iter_mut()
            //                             .flatten(),
            //                         parent_node.id.as_ref(),
            //                         parent_node.depth,
            //                     );
            //
            //                     match &found_parent_res {
            //                         Ok(mut parent) => {
            //                             parent.children_groups.get_mut(target.group_idx()).map(
            //                                 |s| {
            //                                     s.get_mut(target.item_idx()).map(|s| {
            //                                         match &mut s.value.data {
            //                                             MatrixData::Offer(offer) => {
            //                                                 offer.weight = value;
            //                                             }
            //                                             MatrixData::LandingPage(lp) => {
            //                                                 lp.weight = value;
            //                                             }
            //                                             _ => {
            //                                                 notify_danger("Err: y656HG4G");
            //                                             }
            //                                         }
            //                                     })
            //                                 },
            //                             );
            //                         }
            //                         Err(msg) => {
            //                             notify_danger(&msg);
            //                             return false;
            //                         }
            //                     }
            //                 } else {
            //                     notify_danger("No Parent Node");
            //                     return false;
            //                 }
            //             }
            //         },
            //
            //         UpdateMatrix::FillVoid(old, new) => match self.props.seq_type {
            //             SequenceType::OffersOnly => {
            //                 self.props
            //                     .root_matrix
            //                     .borrow_mut()
            //                     .children_groups
            //                     .get_mut(0)
            //                     .map(|s| {
            //                         s.iter_mut()
            //                             .find(|s| s.id() == old.id())
            //                             .map(|s| s.transform_void(new))
            //                     });
            //             }
            //             SequenceType::LandingPageAndOffers => {
            //                 self.props
            //                     .root_matrix
            //                     .borrow_mut()
            //                     .children_groups
            //                     .get_mut(old.group_idx())
            //                     .map(|s| {
            //                         s.iter_mut()
            //                             .find(|s| s.id() == old.id())
            //                             .map(|s| s.transform_void(new))
            //                     });
            //             }
            //             SequenceType::Matrix => {
            //                 if let Some(parent_node) = old.get_parent_node() {
            //                     let found_parent = Matrix::search_next_depth(
            //                         self.props
            //                             .root_matrix
            //                             .borrow_mut()
            //                             .children_groups
            //                             .iter_mut()
            //                             .flatten(),
            //                         parent_node.id.as_ref(),
            //                         parent_node.depth,
            //                     );
            //
            //                     found_parent.map(|s| {
            //                         s.children_groups.get_mut(old.group_idx()).map(|s| {
            //                             s.get_mut(old.item_idx()).map(|s| s.transform_void(new))
            //                         })
            //                     });
            //                 } else {
            //                     notify_danger("No Parent Node");
            //                     return false;
            //                 }
            //             }
            //         },
            //
            //         UpdateMatrix::Remove(item) => {
            //             let parent_matrix = arc!(item
            //                 .read()
            //                 .expect("6hgfegh")
            //                 .value
            //                 .parent_matrix
            //                 .expect("P:G%fg"));
            //
            //             match self.props.seq_type {
            //                 SequenceType::Matrix => {
            //                     self.remove_child(Some(child_to_remove), None, None)
            //                         .map_err(|e| notify_danger(&e));
            //                 }
            //                 SequenceType::OffersOnly => {
            //                     self.remove_child(None, None, Some(*child_to_remove))
            //                         .map_err(|e| notify_danger(&e));
            //                 }
            //                 SequenceType::LandingPageAndOffers => {
            //                     self.remove_child(None, Some(*child_to_remove), None)
            //                         .map_err(|e| notify_danger(&e));
            //                 }
            //             }
            //         }
            //     }
            //     self.props
            //         .sequence_builder_link
            //         .send_message(SeqMsg::UpdateRootMatrix(rc!(self.props.root_matrix)));
            // }
            //
            // Msg::RemoveChild(child_to_remove) => {
            //     self.props
            //         .sequence_builder_link
            //         .send_message(SeqMsg::UpdateRootMatrix(rc!(self.props.root_matrix)));
            // }
            //
            // Msg::AddChild(void_child) => {
            //     match self.props.seq_type {
            //         SequenceType::Matrix => {
            //             self.add_child(Some(void_child), None, None)
            //                 .map_err(|e| notify_danger(&e));
            //         }
            //         SequenceType::OffersOnly => {
            //             self.add_child(None, None, Some(*void_child))
            //                 .map_err(|e| notify_danger(&e));
            //         }
            //         SequenceType::LandingPageAndOffers => {
            //             self.add_child(None, Some(*void_child), None)
            //                 .map_err(|e| notify_danger(&e));
            //         }
            //     }
            //     self.props
            //         .sequence_builder_link
            //         .send_message(SeqMsg::UpdateRootMatrix(rc!(self.props.root_matrix)));
            // }
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut matrix_style = format!("");

        VNode::from(html! {
        // <div class="uk-overflow-auto">
        //     <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
        //         {self.table_head()}
                {self.table_body()}
        //     </table>
        // </div>
            })
    }
}

impl MatrixBuilder {
    pub fn table_body(&self) -> VNode {
        match (
            self.props.seq_type,
            &self.props.local_matrix.read().expect("G%FDrR").data(),
        ) {
            (seq_type, MatrixData::Void) => match seq_type {
                SequenceType::OffersOnly => {
                    let transform_to_offer_cb = self.link.callback(move |offer: Offer| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Offer(offer)))
                    });

                    let remove_callback = self
                        .link
                        .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                    VNode::from(html! {
                                <tr>
                                    <td class="uk-table-expand"><OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb /></td>
                                    <td class="uk-table-shrink"></td>
                                    <td><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                    })
                }

                SequenceType::LandingPageAndOffers => {
                    let transform_to_offer_cb = self.link.callback(move |offer: Offer| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Offer(offer)))
                    });

                    let transform_to_lander_cb = self.link.callback(move |lp: LandingPage| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Lander(lp)))
                    });

                    let remove_callback = self
                        .link
                        .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                    VNode::from(html! {
                                <tr>
                                    <td class="uk-text-truncate"><OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb /></td>
                                    <td class="uk-text-truncate"><LandingPageDropdown state=rc!(self.props.state) eject=transform_to_lander_cb /></td>
                                    <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                    })
                }

                SequenceType::Matrix => {
                    let depth = self.props.local_matrix.read().expect("%GSDF").depth();
                    let depth_border = format!(
                        "border-left-style:solid;border-left-color:{};",
                        color_depth_border(depth)
                    );

                    let transform_to_lander_cb = self.link.callback(move |lp: LandingPage| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Lander(lp)))
                    });

                    let transform_to_offer_cb = self.link.callback(|offer: Offer| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Offer(offer)))
                    });

                    let remove_callback = self
                        .link
                        .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                    if self.props.local_matrix.read().expect("5gsdfg").depth() < 9 {
                        VNode::from(html! {
                                    <tr style=depth_border>
                                        <td class="uk-text-truncate"><OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb /></td>
                                        <td class="uk-text-truncate"><LandingPageDropdown state=rc!(self.props.state) eject=transform_to_lander_cb /></td>
                                        <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    } else {
                        VNode::from(html! {
                                    <tr style=depth_border>
                                        <td class="uk-text-truncate"><OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb /></td>
                                        <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    }
                }
            },

            (SequenceType::OffersOnly, MatrixData::Source) => {
                let mut offer_children_nodes = VList::new();
                let matrix_handle = self.props.local_matrix.read().expect("GTRdsfg");
                let offer_children = matrix_handle.children_groups.get(0).unwrap();
                let add_callback = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(0)));

                for matrix in offer_children {
                    let local_matrix = arc!(matrix);

                    offer_children_nodes.push(html! {
                        <MatrixBuilder
                        root_matrix=arc!(self.props.root_matrix)
                        local_matrix=local_matrix
                        state=rc!(self.props.state)
                        seq_type=SequenceType::OffersOnly
                        sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                        />
                    });
                }

                VNode::from(html! {
                <>
                    <button onclick=add_callback class="uk-button uk-button-small uk-button-primary">{"Add Offer"}</button>
                    <div class="uk-overflow-auto">
                        <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
                            {self.table_head()}
                                    <tbody>
                                        {offer_children_nodes}
                                    </tbody>
                        </table>
                    </div>
                </>
                    })
            }

            (SequenceType::OffersOnly, MatrixData::Offer(offer)) => {
                let blur_update_weight_callback = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Weight));
                let update_weight_callback =
                    self.link.callback(move |i: InputData| Msg::UpdateWeight(i));
                let remove_callback = self
                    .link
                    .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Remove));
                let weight_val = self.weight_buff.to_string();

                VNode::from(html! {
                            <tr>
                                <td class="uk-text-truncate" uk-tooltip=format!("title: {};", &offer.name) ><span>{format!("{}", &offer.name)}</span></td>
                                <td class="uk-text-truncate"><input class="uk-input" value=weight_val oninput=update_weight_callback onblur=blur_update_weight_callback placeholder="Weight" /></td>
                                <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                            </tr>
                })
            }

            (SequenceType::LandingPageAndOffers, MatrixData::Source) => {
                let mut nodes = VList::new();
                let matrix_handle = self.props.local_matrix.read().expect("g546sdfg");
                let first_group_add_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(0)));

                nodes.push(html! {
                            <tr>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=first_group_add_cb class="uk-button uk-button-large uk-button-primary">{"Add Element (Lander or Offer)"}</button></td>
                            </tr>
                });

                let first_group_nodes = matrix_handle.children_groups.get(0).unwrap();

                for (item_idx, item) in first_group_nodes.iter().enumerate() {
                    let local_matrix = arc!(item);

                    nodes.push(html! {
                            <MatrixBuilder
                            root_matrix=arc!(self.props.root_matrix)
                            local_matrix=local_matrix
                            state=rc!(self.props.state)
                            seq_type=SequenceType::LandingPageAndOffers
                            sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                            />
                    });
                }
                nodes.push(html! {{divider!(2)}});

                let rest_of_groups = matrix_handle
                    .children_groups
                    .iter()
                    .enumerate()
                    .filter(|(group_idx, _)| *group_idx != 0);

                for (group_idx, group) in rest_of_groups {
                    let add_offer_to_group_cb = self
                        .link
                        .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(group_idx)));

                    nodes.push(html! {
                            <tr>
                                <td class="uk-text-nowrap uk-text-center">{"Offer Groups 2 Debug..."}</td>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=add_offer_to_group_cb class="uk-button uk-button-large uk-button-primary">{"Add Offer"}</button></td>
                            </tr>
                });

                    for (item_idx, item) in group.iter().enumerate() {
                        let local_matrix = arc!(item);

                        nodes.push(html! {
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=local_matrix
                                state=rc!(self.props.state)
                                seq_type=SequenceType::LandingPageAndOffers
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
                        });
                    }
                }

                VNode::from(html! {
                <div class="uk-overflow-auto">
                    <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
                        {self.table_head()}
                                <tbody>
                                    {nodes}
                                </tbody>
                    </table>
                </div>
                })
            }

            (SequenceType::LandingPageAndOffers, matrix_data) => {
                let matrix_handle = self.props.local_matrix.read().expect("g546sdfg");
                let rm_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                let oninput_update_weight_cb =
                    self.link.callback(move |i: InputData| Msg::UpdateWeight(i));

                let local_matrix = arc!(self.props.local_matrix);
                let onblur_update_weight_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Weight));
                let weight_value = self.weight_buff.to_string();

                match matrix_data {
                    MatrixData::LandingPage(lp) => {
                        let num_cta = lp.number_of_calls_to_action;

                        VNode::from(html! {
                                    <tr>
                                        <td class="uk-text-truncate">{format!("{}", &lp.name)}</td>
                                        <td class="uk-text-nowrap">{"Lander"}</td>
                                        <td class="uk-text-nowrap">{num_cta}</td>
                                        <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight_value onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                        <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    }
                    MatrixData::Offer(offer) => VNode::from(html! {
                                <tr>
                                    <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                                    <td class="uk-text-nowrap">{"Offer"}</td>
                                    <td class="uk-text-nowrap">{"NA"}</td>
                                    <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight_value onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                    <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                    }),
                    _ => {
                        html! {}
                    }
                }
            }

            (SequenceType::Matrix, MatrixData::Source) => {
                let mut nodes = VList::new();
                let matrix_handle = self.props.local_matrix.read().expect("GTsfdg");
                let source_groups = matrix_handle.children_groups.iter().enumerate();

                for (group_idx, group) in source_groups {
                    let add_cb = self
                        .link
                        .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(group_idx)));

                    nodes.push(html! {
                            <tr>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=add_cb class="uk-button uk-button-large uk-button-primary">{"Add Element (Lander or Offer)"}</button></td>
                            </tr>
                });

                    for (item_idx, item) in group.iter().enumerate() {
                        let local_matrix = arc!(item);

                        nodes.push(html! {
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=local_matrix
                                state=rc!(self.props.state)
                                seq_type=SequenceType::Matrix
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
                    });
                    }
                }

                VNode::from(html! {
                <div class="uk-overflow-auto">
                    <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
                        {self.table_head()}
                                <tbody>
                                    {nodes}
                                </tbody>
                    </table>
                </div>
                })
            }

            (SequenceType::Matrix, MatrixData::Offer(offer)) => {
                let matrix = self.props.local_matrix.read().expect("tgfgF45F");
                let depth = matrix.depth();
                let depth_border = format!(
                    "border-left-style:solid;border-left-color:{};",
                    color_depth_border(depth)
                );
                let weight = offer.weight.to_string();

                let oninput_update_weight_cb =
                    self.link.callback(move |i: InputData| Msg::UpdateWeight(i));

                let local_matrix = arc!(self.props.local_matrix);
                let onblur_update_weight_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Weight));

                let local_matrix = arc!(self.props.local_matrix);
                let rm_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                VNode::from(html! {
                                <tr style=depth_border>
                                    <td class="uk-text-nowrap">{depth}</td>
                                    <td class="uk-text-nowrap">{"Offer"}</td>
                                    <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                                    <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                    <td class="uk-text-nowrap">{"NA"}</td>
                                    <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                })
            }

            (SequenceType::Matrix, MatrixData::LandingPage(lp)) => {
                let mut nodes = VList::new();
                let matrix = self.props.local_matrix.read().expect("tgfgF45F");
                let depth = matrix.depth();
                let depth_border = format!(
                    "border-left-style:solid;border-left-color:{};",
                    color_depth_border(depth)
                );
                let weight = lp.weight.to_string();
                let ctas = lp.number_of_calls_to_action.to_string();
                let oninput_update_weight_cb =
                    self.link.callback(move |i: InputData| Msg::UpdateWeight(i));
                let onblur_update_weight_cb = self
                    .link
                    .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Weight));
                let rm_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                nodes.push(html! {{divider!(2)}});
                nodes.push(html! {
                                <tr style=depth_border>
                                    <td class="uk-text-nowrap">{depth}</td>
                                    <td class="uk-text-nowrap">{"Lander"}</td>
                                    <td class="uk-text-truncate">{format!("{}", &lp.name)}</td>
                                    <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                    <td class="uk-text-nowrap">{"NA"}</td>
                                    <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                });

                for (group_idx, group) in matrix.children_groups.iter().enumerate() {
                    nodes.push(html! {{divider!(2)}});

                    for (item_idx, item) in group.iter().enumerate() {
                        let local_matrix = arc!(item);

                        nodes.push(html! {
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=local_matrix
                                state=rc!(self.props.state)
                                seq_type=SequenceType::Matrix
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
                    });
                    }
                }
                nodes.push(html! {{divider!(2)}});

                VNode::from(nodes)
            }

            _ => VNode::from(html! {}),
        }
    }

    pub fn table_head(&self) -> VNode {
        match (
            self.props.seq_type,
            self.props.local_matrix.read().expect("5tgFt5RF").data(),
        ) {
            (SequenceType::OffersOnly, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
                        <th class="uk-table-shrink">{"Weight"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            (SequenceType::LandingPageAndOffers, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Type"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"CTA's"}</th>
                        <th class="uk-table-shrink">{"Weight"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            (SequenceType::Matrix, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{format!("Depth")}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Type"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
                        <th class="uk-table-shrink">{"Weight"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"CTA's"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            _ => VNode::from(html! {}),
        }
    }
}

// pub fn add_child(
//     &self,
//     matrix_child_target_to_add: Option<Arc<RwLock<Matrix>>>,
//     landing_page_source_child_target_to_add: Option<Matrix>,
//     offer_source_child_target_to_add: Option<Matrix>,
// ) -> Result<usize, String> {
//     match self.props.seq_type {
//         SequenceType::Matrix => {
//             self.add_child_to_matrix(matrix_child_target_to_add.expect("F453sdF"))
//         }
//
//         SequenceType::LandingPageAndOffers => self.add_landing_page_child_to_source(
//             landing_page_source_child_target_to_add.expect("5gfgDF43d"),
//         ),
//
//         SequenceType::OffersOnly => self.add_child_to_offer_source(
//             offer_source_child_target_to_add.expect("%gfr$4rfdg54FR"),
//         ),
//     }
// }

// pub fn add_child_to_matrix(&self, child_target: Arc<RwLock<Matrix>>) -> Result<usize, String> {
//     if let Some(parent_node) = child_target.get_parent_node() {
//         let target_parent_depth = parent_node.depth;
//         let target_parent_id = parent_node.id.as_ref();
//         let child_group_index = child_target.group_idx();
//         let child_item_index = child_target.item_idx();
//         let mut root = self.props.root_matrix.borrow_mut();
//
//         let found_parent = Matrix::search_next_depth(
//             root.children_groups.iter_mut().flatten(),
//             target_parent_id,
//             target_parent_depth,
//         );
//
//         match found_parent {
//             Ok(parent_target) => {
//                 if let Some(target_group) =
//                     parent_target.children_groups.get_mut(child_group_index)
//                 {
//                     let bf = target_group.len();
//                     target_group.insert(child_item_index, *child_target);
//                     parent_target.synchronize_matrix_child_groups();
//                     Ok(target_group.len() - bf)
//                 } else {
//                     return Err("Invalid gp idx".to_string());
//                 }
//             }
//             Err(e) => return Err(e),
//         }
//     } else {
//         let mut root = self.props.root_matrix.borrow_mut();
//         let child_group_index = child_target.group_idx();
//         let child_item_index = child_target.item_idx();
//
//         if let Some(group) = root.children_groups.get_mut(child_group_index) {
//             let bf = group.len();
//             group.insert(child_item_index, *child_target);
//             Ok(group.len() - bf)
//         } else {
//             Err("Invalid group idx".to_string())
//         }
//     }
// }

// pub fn add_landing_page_child_to_source(
//     &self,
//     landing_page_source_child_target: Matrix,
// ) -> Result<usize, String> {
//     let group_idx = landing_page_source_child_target.group_idx();
//     let item_idx = landing_page_source_child_target.item_idx();
//
//     let mut source = self.props.root_matrix.borrow_mut();
//     if let Some(group) = source.children_groups.get_mut(group_idx) {
//         let bf = group.len();
//         group.insert(item_idx, landing_page_source_child_target);
//         source.root_synchronize_landing_page_child_groups();
//         Ok(group.len() - bf)
//     } else {
//         Err(new_string!("group not found"))
//     }
// }
//
// pub fn add_child_to_offer_source(
//     &self,
//     offer_source_child_target: Matrix,
// ) -> Result<usize, String> {
//     let mut source = self.props.root_matrix.borrow_mut();
//     let item_index = offer_source_child_target.item_idx();
//
//     if let Some(g) = source.children_groups.get_mut(0) {
//         let bf = g.len();
//         g.insert(item_index, offer_source_child_target);
//         Ok(g.len() - bf)
//     } else {
//         Err(new_string!("Group Not Found"))
//     }
// }
//
// pub fn remove_child(
//     &self,
//     matrix_child_target: Option<Arc<RwLock<Matrix>>>,
//     landing_page_source_child_target: Option<Matrix>,
//     offer_source_child_target: Option<Matrix>,
// ) -> Result<Matrix, String> {
//     match self.props.seq_type {
//         SequenceType::Matrix => {
//             self.remove_child_for_matrix(matrix_child_target.expect("G%sfdg5f"))
//         }
//
//         SequenceType::LandingPageAndOffers => self.remove_landing_page_child_from_source(
//             landing_page_source_child_target.expect("54Gf54FG"),
//         ),
//
//         SequenceType::OffersOnly => {
//             self.remove_offer_child_from_source(offer_source_child_target.expect("G5gFrr$f"))
//         }
//     }
// }
//
// pub fn remove_landing_page_child_from_source(
//     &self,
//     landing_page_source_child_target: Matrix,
// ) -> Result<Matrix, String> {
//     let group_idx = landing_page_source_child_target.group_idx();
//     let item_idx = landing_page_source_child_target.item_idx();
//
//     let mut source = self.props.root_matrix.borrow_mut();
//     if let Some(group) = source.children_groups.get_mut(group_idx) {
//         if let Some(i) = group.get(item_idx) {
//             let res = group.remove(item_idx);
//             source.root_synchronize_landing_page_child_groups();
//             Ok(res)
//         } else {
//             Err(new_string!("item not found"))
//         }
//     } else {
//         Err(new_string!("group not found"))
//     }
// }
//
// pub fn remove_offer_child_from_source(
//     &self,
//     offer_source_child_target: Matrix,
// ) -> Result<Matrix, String> {
//     let mut source = self.props.root_matrix.borrow_mut();
//     let item_index = offer_source_child_target.item_idx();
//     if let Some(g) = source.children_groups.get_mut(0) {
//         if let Some(i) = g.get(item_index) {
//             Ok(g.remove(item_index))
//         } else {
//             Err(new_string!("Item index not found"))
//         }
//     } else {
//         Err(new_string!("Group Not Found"))
//     }
// }

//     pub fn remove_child_for_matrix(
//         &self,
//         child_target: Arc<RwLock<Matrix>>,
//     ) -> Result<Matrix, String> {
//         if let Some(parent_node) = child_target.get_parent_node() {
//             let target_parent_depth = parent_node.depth;
//             let target_parent_id = parent_node.id.as_ref();
//             let child_group_index = child_target.group_idx();
//             let child_item_index = child_target.item_idx();
//             let mut root = self.props.root_matrix.borrow_mut();
//
//             let found_parent = Matrix::search_next_depth(
//                 root.children_groups.iter_mut().flatten(),
//                 target_parent_id,
//                 target_parent_depth,
//             );
//
//             match found_parent {
//                 Ok(parent_target) => {
//                     if let Some(target_group) =
//                         parent_target.children_groups.get_mut(child_group_index)
//                     {
//                         let res = target_group.remove(child_item_index);
//                         parent_target.synchronize_matrix_child_groups();
//                         Ok(res)
//                     } else {
//                         return Err("Invalid gp idx".to_string());
//                     }
//                 }
//                 Err(e) => return Err(e),
//             }
//         } else {
//             let mut root = self.props.root_matrix.borrow_mut();
//             let child_group_index = child_target.group_idx();
//             let child_item_index = child_target.item_idx();
//
//             if let Some(group) = root.children_groups.get_mut(child_group_index) {
//                 Ok(group.remove(child_item_index))
//             } else {
//                 Err("Invalid group idx".to_string())
//             }
//         }
//     }
// }

pub fn hide_children(props: &Props) -> bool {
    if props.local_matrix.read().expect("%TG$FG").value.depth == 0 {
        false
    } else {
        true
    }
}

pub fn color_depth_border(depth: usize) -> &'static str {
    match depth {
        0 => DEPTH_0,
        1 => DEPTH_1,
        2 => DEPTH_2,
        3 => DEPTH_3,
        4 => DEPTH_4,
        5 => DEPTH_5,
        6 => DEPTH_6,
        7 => DEPTH_7,
        8 => DEPTH_8,
        9 => DEPTH_9,
        _ => DEPTH_0,
    }
}
