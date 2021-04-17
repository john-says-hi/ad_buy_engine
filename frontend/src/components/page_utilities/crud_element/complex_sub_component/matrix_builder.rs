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
use crate::components::page_utilities::crud_element::crud_funnels::CRUDFunnel;

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
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub children_hidden: bool,
    pub weight_buff: u8,
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
                            if let SequenceType::LandingPageAndOffers = self.props.seq_type {
                                let offer_groups =
                                    self.props.root_matrix.read().unwrap().children_groups.len()
                                        - 1;
                                let ctas = lp.number_of_calls_to_action as usize;

                                if ctas > offer_groups {
                                    let difference_to_add = ctas - offer_groups;
                                    for i in 0..difference_to_add {
                                        let new_group_idx = self
                                            .props
                                            .root_matrix
                                            .read()
                                            .unwrap()
                                            .children_groups
                                            .len();
                                        notify_danger(
                                            format!("New Group Idx: {}", &new_group_idx).as_str(),
                                        );
                                        self.props
                                            .root_matrix
                                            .write()
                                            .unwrap()
                                            .children_groups
                                            .push(vec![Arc::new(RwLock::new(Matrix::void(
                                                Some(arc!(self.props.root_matrix)),
                                                new_group_idx,
                                                0,
                                                1,
                                            )))])
                                    }
                                }
                                self.props.local_matrix.write().expect("G53greg").value.data =
                                    MatrixData::LandingPage(lp);
                            } else {
                                let local_group_idx =
                                    self.props.local_matrix.read().unwrap().value.group_idx;
                                let ctas = lp.number_of_calls_to_action as usize;
                                self.props.local_matrix.write().expect("G53greg").value.data =
                                    MatrixData::LandingPage(lp);
                                for i in 0..ctas {
                                    let new_group_idx = i + 1;
                                    let depth =
                                        self.props.local_matrix.read().unwrap().value.depth + 1;
                                    self.props
                                        .local_matrix
                                        .write()
                                        .unwrap()
                                        .children_groups
                                        .push(vec![Arc::new(RwLock::new(Matrix::void(
                                            Some(arc!(self.props.local_matrix)),
                                            new_group_idx,
                                            0,
                                            depth,
                                        )))])
                                }
                            }
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
                                if group.len() == 0 {
                                    group.push(Arc::new(RwLock::new(Matrix::void(
                                        Some(arc!(parent_node)),
                                        target_group_idx,
                                        0,
                                        1,
                                    ))));
                                }
                            } else {
                                notify_danger("Group not found")
                            };

                            // Remove Offer groups if needed
                            if let SequenceType::LandingPageAndOffers = self.props.seq_type {
                                let highest_cta = Matrix::highest_cta(
                                    &self
                                        .props
                                        .root_matrix
                                        .read()
                                        .unwrap()
                                        .children_groups
                                        .get(0)
                                        .unwrap(),
                                );
                                let num_of_offer_groups =
                                    self.props.root_matrix.read().unwrap().children_groups.len()
                                        - 1;

                                if num_of_offer_groups > highest_cta {
                                    let mut diff_to_rm = num_of_offer_groups - highest_cta;
                                    for i in (0..diff_to_rm).rev() {
                                        let idx_to_rm = i + 1;
                                        // notify_danger(
                                        //     format!("Idx to Remove: {}", &idx_to_rm).as_str(),
                                        // );
                                        self.props
                                            .root_matrix
                                            .write()
                                            .unwrap()
                                            .children_groups
                                            .remove(idx_to_rm);
                                    }
                                }
                            }
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
                        } else {
                            notify_danger("No group, Adding new...");
                            let new_group_idx = local_matrix_handle.children_groups.len();
                            local_matrix_handle
                                .children_groups
                                .push(vec![Arc::new(RwLock::new(Matrix::void(
                                    Some(arc!(parent_node)),
                                    new_group_idx,
                                    0,
                                    dept + 1,
                                )))]);
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
            {self.table_body()}
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
                                <tr uk-tooltip="title:Please Fill Out;" style="background:#ffcccb;" >
                                    <td class="uk-table-expand">
                                        {label!("Offer")}
                                        <OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb />
                                    </td>
                                    <td class="uk-table-shrink"></td>
                                    <td><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                </tr>
                    })
                }

                SequenceType::LandingPageAndOffers => {
                    let group_idx = self.props.local_matrix.read().unwrap().value.group_idx;

                    let transform_to_offer_cb = self.link.callback(move |offer: Offer| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Offer(offer)))
                    });
                    let transform_to_lander_cb = self.link.callback(move |lp: LandingPage| {
                        Msg::UpdateMatrix(UpdateMatrix::FillVoid(Transform::Lander(lp)))
                    });
                    let remove_callback = self
                        .link
                        .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Remove));

                    if group_idx == 0 {
                        VNode::from(html! {
                                    <tr uk-tooltip="title:Please Fill Out;" style="background:#ffcccb;">
                                        <td class="uk-text-expand">
                                            {label!("Offer")}
                                            <OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb />
                                        </td>
                                        <td class="uk-text-expand">
                                            {label!("Lander")}
                                            <LandingPageDropdown state=rc!(self.props.state) eject=transform_to_lander_cb />
                                        </td>
                                        <td></td>
                                        <td></td>
                                        <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    } else {
                        VNode::from(html! {
                                    <tr uk-tooltip="title:Please Fill Out;" style="background:#ffcccb;">
                                        <td class="uk-text-expand">
                                            {label!("Offer")}
                                            <OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb />
                                        </td>
                                        <td></td>
                                        <td></td>
                                        <td></td>
                                        <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    }
                }

                SequenceType::Matrix => {
                    let depth = self.props.local_matrix.read().expect("%GSDF").depth();
                    let depth_border = format!(
                        "border-left-style:solid;border-left-color:{};background:#ffcccb;",
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
                                    <tr style=depth_border uk-tooltip="title:Please Fill Out;">
                                        <td class="uk-text-expand">
                                            {label!("Offer")}
                                            <OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb />
                                        </td>
                                        <td class="uk-text-expand">
                                            {label!("Lander")}
                                            <LandingPageDropdown state=rc!(self.props.state) eject=transform_to_lander_cb />
                                        </td>
                                        <td></td>
                                        <td></td>
                                        <td></td>
                                        <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    } else {
                        VNode::from(html! {
                                    <tr style=depth_border uk-tooltip="title:Please Fill Out;">
                                        <td class="uk-text-expand">
                                            {label!("Offer")}
                                            <OfferDropdown state=rc!(self.props.state) eject=transform_to_offer_cb />
                                        </td>
                                        <td></td>
                                        <td></td>
                                        <td></td>
                                        <td></td>
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

                for (group_idx, group) in self
                    .props
                    .local_matrix
                    .read()
                    .unwrap()
                    .children_groups
                    .iter()
                    .enumerate()
                {
                    let mut items = VList::new();

                    for (item_idx, item) in group.iter().enumerate() {
                        let local_matrix = arc!(item);
                        items.push(html! {
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=local_matrix
                                state=rc!(self.props.state)
                                seq_type=SequenceType::LandingPageAndOffers
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
                        });
                    }

                    let btn_txt = if group_idx == 0 {
                        format!("Add Depth 1 Element")
                    } else {
                        format!("Add to Offer Group {}", group_idx)
                    };
                    let top_divider = if group_idx == 0 {
                        VNode::from(html! {})
                    } else {
                        VNode::from(html! {{divider!(2)}})
                    };
                    let add_cb = self
                        .link
                        .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(group_idx)));
                    let group_headline = if group_idx == 0 {
                        VNode::from(html! {<h4 class="uk-margin-small">{"Depth 1 Elements"}</h4>})
                    } else {
                        if group_idx == 1 {
                            VNode::from(html! {
                                <>
                            <h4 class="uk-margin-small">{"Depth 2 Elements"}</h4>
                            <h5 class="uk-margin-small">{format!("Offer Group {}", group_idx)}</h5>
                                </>
                            })
                        } else {
                            VNode::from(
                                html! {<h5 class="uk-margin-small">{format!("Offer Group {}", group_idx)}</h5>},
                            )
                        }
                    };

                    nodes.push(VNode::from(html! {
                        <>
                            // {top_divider}
                            {group_headline}
                            <button onclick=add_cb class="uk-button uk-button-small uk-button-primary">{btn_txt}</button>
                            <div class="uk-overflow-auto">
                                <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
                                    {self.table_head()}
                                            <tbody>
                                                {items}
                                            </tbody>
                                </table>
                            </div>
                        </>
                    }));
                }

                VNode::from(nodes)
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

                let group_idx = matrix_handle.value.group_idx;
                let depth_border = if group_idx == 0 {
                    format!(
                        "border-left-style:solid;border-left-color:{};",
                        color_depth_border(1)
                    )
                } else {
                    format!(
                        "border-left-style:solid;border-left-color:{};",
                        color_depth_border(2)
                    )
                };

                match matrix_data {
                    MatrixData::LandingPage(lp) => {
                        let num_cta = lp.number_of_calls_to_action;

                        VNode::from(html! {
                                    <tr style=depth_border>
                                        <td class="uk-text-truncate" uk-tooltip={format!("title:{};", &lp.name)}>{format!("{}", &lp.name)}</td>
                                        <td class="uk-text-nowrap">{"Lander"}</td>
                                        <td class="uk-text-nowrap">{num_cta}</td>
                                        <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight_value onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                        <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                                    </tr>
                        })
                    }
                    MatrixData::Offer(offer) => VNode::from(html! {
                                <tr style=depth_border>
                                    <td class="uk-text-truncate" uk-tooltip={format!("title:{};", &offer.name)}>{format!("{}", &offer.name)}</td>
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
                let mut matrices = VList::new();
                let matrix_handle = self.props.local_matrix.read().expect("GTsfdg");

                for (idx, source_matrix) in matrix_handle.children_groups.iter().enumerate() {
                    if let Some(local_matrix) = source_matrix.get(0) {
                        matrices.push(VNode::from(html! {
                    <div class="uk-margin-top">
                              {label!(&format!("Matrix #{}", idx + 1))}
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=arc!(local_matrix)
                                state=rc!(self.props.state)
                                seq_type=SequenceType::Matrix
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
                    </div>
                    }));
                    }
                }

                let new_group_idx = matrix_handle.children_groups.len();
                let add_cb = self
                    .link
                    .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Add(new_group_idx)));
                VNode::from(html! {
                    <>
                    <button onclick=add_cb class="uk-button uk-button-small uk-button-primary">{"Add Matrix"}</button>
                    {matrices}
                    </>
                })
            }

            (SequenceType::Matrix, MatrixData::LandingPage(lp)) => {
                html! {
                    {self.matrix_lander_base(lp)}
                }
            }

            (SequenceType::Matrix, MatrixData::Offer(offer)) => VNode::from(html! {
                {self.matrix_offer_row(offer)}
            }),

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
                            <div class="uk-grid-column-small uk-grid-row-small uk-child-width-1-6 uk-no-wrap uk-text-center" uk-grid="">
                                // <div>
                            // {"Step"}
                            //     </div>
                                <div>
                            {"Depth"}
                                </div>
                                <div>
                            {"Type"}
                                </div>
                                <div>
                            {"Name"}
                                </div>
                                <div>
                            {"Weight"}
                                </div>
                                <div>
                            {"CTAs"}
                                </div>
                                <div>
                            {"Remove"}
                                </div>
                            </div>

            }),

            _ => VNode::from(html! {}),
        }
    }
}

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

pub fn margin_depth_indent(depth: usize) -> i32 {
    match depth {
        0 => 0,
        1 => 0,
        2 => 5,
        3 => 10,
        4 => 20,
        5 => 25,
        6 => 30,
        7 => 35,
        8 => 40,
        9 => 45,
        _ => 50,
    }
}
