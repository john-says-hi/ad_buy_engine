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
use web_sys::{Element, set_cross_origin};
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::{VNode, VList};
use yew_services::storage::Area;
use yew_services::StorageService;
use serde::de::Unexpected::Seq;

pub type RootMatrix = Rc<RefCell<Matrix>>;

pub enum Msg {
    UpdateMatrix(RootMatrix),
    RemoveChild(Rc<Matrix>),// match seq type to control dynamic child group dropdown/>?
    AddChild(Rc<Matrix>, usize), // here too
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
            Msg::RemoveChild(rm) => {
            }
            
            Msg::AddChild(parent_node, group_pos)=>{
            
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
        match (self.props.seq_type, self.props.local_matrix.data()) {
            /// add depth to body of lps offers and child comps below parrent && add to header too, matrix only tho!
            (SequenceType::OffersOnly, MatrixData::Source) => {
                let offer_children = self.props.local_matrix.children_groups.get(0).unwrap();
                let mut offer_children_nodes = VList::new();
                self.props.
                for child in offer_children {
                    offer_children_nodes.push(html!{
                        <MatrixBuilder root_matrix=rc!(self.props.root_matrix) local_matrix=rc!(self.props.local_matrix) state=rc!(self.props.state) seq_type=SequenceType::OffersOnly transmit=self.link.callback(Msg::UpdateMatrix) sequence_builder_link=Rc::clone(&self.props.sequence_builder_link) />
                    })
                }
                
                VNode::from(html! {
                    <tbody>
                        <tr>
                            <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                            <td class="uk-text-nowrap">{"Lorem ipsum dolor"}</td>
                        </tr>
                    </tbody>
            })
            },
            
            (SequenceType::OffersOnly, MatrixData::Offer(offer)) => {
                VNode::from(html! {
                    // <tbody>
                        <tr>
                            <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                            <td class="uk-text-nowrap">{"Lorem ipsum dolor"}</td>
                        </tr>
                    // </tbody>
            })
            },

            _ => VNode::from(html! {}),
        }
    }

    pub fn table_head(&self) -> VNode {
        match (self.props.seq_type, self.props.local_matrix.data()) {
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

    pub fn remove_child(&self, matrix_child_target: Option<Rc<Matrix>>, landing_page_source_child_target: Option<Matrix>, offer_source_child_target: Option<Matrix>) -> Result<Matrix, String> {
        match self.props.seq_type {
        SequenceType::Matrix =>{
                self.remove_child_for_matrix(matrix_child_target?)
            }
        
        SequenceType::LandingPageAndOffers =>{
            self.remove_landing_page_child_from_source(landing_page_source_child_target?)
        }
    
        SequenceType::OffersOnly =>{
            self.remove_offer_child_from_source(offer_source_child_target?)
        }
    }
    }
    
    pub fn remove_landing_page_child_from_source(&self, landing_page_source_child_target: Matrix) -> Result<Matrix, String> {
        let group_idx = landing_page_source_child_target.group_idx();
        let item_idx=landing_page_source_child_target.item_idx();
        
        let mut source = self.props.root_matrix.borrow_mut();
        if let Some(group)=source.children_groups.get_mut(group_idx) {
            if let Some(i) = group.get(item_idx) {
                let res =group.remove(item_idx);
                source.root_synchronize_landing_page_child_groups();
                Ok(res)
            } else {
                Err(new_string!("item not found"))
            }
        } else {
            Err(new_string!("group not found"))
        }
    }
    
    pub fn remove_offer_child_from_source(&self, offer_source_child_target: Matrix) -> Result<Matrix, String> {
        let mut source = self.props.root_matrix.borrow_mut();
        let item_index = offer_source_child_target.item_idx();
        if let Some(g)=source.children_groups.get_mut(0){
            if let Some(i)=g.get(item_index) {
                Ok(g.remove(item_index))
            } else {
                Err(new_string!("Item index not found"))
            }
        } else {
            Err(new_string!("Group Not Found"))
        }
    }
    
    pub fn remove_child_for_matrix(&self, child_target: Rc<Matrix>) -> Result<Matrix, String> {
        if let Some(parent_node) = child_target.get_parent_node() {
            let parent_child_target_depth = parent_node.depth;
            let parent_child_target_id = parent_node.id.as_ref();
            let child_group_index = child_target.group_idx();
            let child_item_index = child_target.item_idx();
            let mut root = self.props.root_matrix.borrow_mut();
    
            let found_parent = Matrix::search_next_depth(
                root.children_groups.iter_mut().flatten(),
                parent_child_target_id,
                parent_child_target_depth,
            );
    
            match found_parent {
                Ok(parent_target) => {
                    if let Some(target_group) = parent_target.children_groups.get_mut(child_group_index) {
                        let res = target_group.remove(child_item_index);
                        parent_target.synchronize_matrix_child_groups();
                        Ok(res)
                    } else {
                        return Err("Invalid gp idx".to_string());
                    }
                }
                Err(e) => return Err(e),
            }
            
        } else {
            let mut root = self.props.root_matrix.borrow_mut();
            let child_group_index = child_target.group_idx();
            let child_item_index = child_target.item_idx();

            if let Some(group) = root.children_groups.get_mut(child_group_index) {
                Ok(group.remove(child_item_index))
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
