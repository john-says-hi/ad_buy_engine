use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_sequence_builder::RHSSequenceBuilder;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::constant::{
    COLOR_BLUE, DEPTH_0, DEPTH_1, DEPTH_2, DEPTH_3, DEPTH_4, DEPTH_5, DEPTH_6, DEPTH_7, DEPTH_8,
    DEPTH_9,
};
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::matrix::{Matrix, MatrixData};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_services::storage::Area;
use yew_services::StorageService;

pub type RootMatrix = Rc<RefCell<Matrix>>;

pub enum Msg {
    RemoveItem(Matrix),
    AddItem(Matrix),
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub root_matrix: RootMatrix,
    pub local_matrix: Rc<Matrix>,
    pub state: STATE,
    pub seq_type: SequenceType,
    pub transmit: Callback<RootMatrix>,
    pub sequence_builder_link: Rc<Scope<RHSSequenceBuilder>>,
}

pub struct MatrixBuilder {
    link: ComponentLink<Self>,
    props: Props,
    children_hidden: bool,
}

impl Component for MatrixBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            children_hidden: hide_children(&props),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RemoveItem(rm) => {
                let mut porthole = Rc::clone(&self.props.root_matrix);
            }
            Msg::Ignore => {}
            _ => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.local_matrix.eq(&self.props.local_matrix) {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let mut matrix_style = format!("");

        VNode::from(html! {
        <div class="uk-overflow-auto">
            <table class="uk-table uk-table-hover uk-table-middle uk-table-divider">
                {self.table_head()}
                {self.table_body()}
            </table>
        </div>
            })
    }
}

impl MatrixBuilder {
    pub fn table_body(&self) -> VNode {
        match (self.props.seq_type, self.props.local_matrix.get_data()) {
            /// add depth to body of lps offers and child comps below parrent && add to header too, matrix only tho!
            (SequenceType::OffersOnly, MatrixData::Offer(offer)) => VNode::from(html! {
                    <tbody>
                        <tr>
                            <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                            <td class="uk-text-nowrap">{"Lorem ipsum dolor"}</td>
                        </tr>
                    </tbody>
            }),

            _ => VNode::from(html! {}),
        }
    }

    pub fn table_head(&self) -> VNode {
        match (self.props.seq_type, self.props.local_matrix.get_data()) {
            (SequenceType::OffersOnly, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
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
                        {self.weight_header()}
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            (SequenceType::Matrix, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{format!("Depth: {}",self.props.local_matrix.depth())}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{format!("{}", if self.children_hidden{"Show"}else{"Hide"})}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Type"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
                        {self.weight_header()}
                        <th class="uk-table-shrink uk-text-nowrap">{"CTA's"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            _ => VNode::from(html! {}),
        }
    }

    pub fn weight_header(&self) -> VNode {
        if self.props.local_matrix.has_children_in_groups() {
            VNode::from(html! {<th class="uk-table-shrink uk-text-nowrap">{"Weight"}</th>})
        } else {
            html! {}
        }
    }

    pub fn remove_child(&self, target: Rc<Matrix>) -> Result<Matrix, String> {
        if let Some(parent_node) = target.get_parent_node() {
            let target_depth = parent_node.depth;
            let target_id = parent_node.id.as_ref();
            let group_index = target.group_idx();
            let item_index = target.item_idx();
            let mut root = self.props.root_matrix.borrow_mut();

            let parent_node = Matrix::search_next_depth(
                root.children_groups.iter_mut().flatten(),
                target_id,
                target_depth,
            );
            match parent_node {
                Ok(parent) => {
                    if let Some(group) = parent.children_groups.get_mut(group_index) {
                        Ok(group.remove(item_index))
                    } else {
                        Err("Invalid gp idx".to_string())
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            let mut root = self.props.root_matrix.borrow_mut();
            let group_index = target.group_idx();
            let item_index = target.item_idx();

            if let Some(group) = root.children_groups.get_mut(group_index) {
                Ok(group.remove(item_index))
            } else {
                Err("Invalid group idx".to_string())
            }
        }
    }
}

pub fn hide_children(props: &Props) -> bool {
    if props.local_matrix.value.depth == 0 {
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
