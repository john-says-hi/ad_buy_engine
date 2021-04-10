use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_sequence_builder::RHSSequenceBuilder;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
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
use web_sys::{set_cross_origin, Element};
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_services::storage::Area;
use yew_services::StorageService;
use crate::notify_danger;
use std::sync::Arc;

pub type RootMatrix = Rc<RefCell<Matrix>>;

pub enum Msg {
    UpdateRootMatrix(RootMatrix),
    UpdateMatrix(UpdateMatrix),
    RemoveChild(Rc<Matrix>),
    AddChild(Rc<Matrix>
    ),
    UpdateWeight(InputData),
    Ignore,
}

pub enum UpdateMatrix {
    Weight(Rc<Matrix>, u8),
    FillVoid(Rc<Matrix>, Transform)
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
    weight_buff: u8,
}

impl Component for MatrixBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            children_hidden: hide_children(&props),
            link,
            props,
            weight_buff: 100,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            
            Msg::UpdateWeight(i)=>{
                if let Ok(num) = i.value.parse::<u8>() {
                    self.weight_buff=num;
                } else {
                    notify_danger("Invalid")
                }
            }
            
            Msg::UpdateMatrix(instruct)=>{
                match instruct {
                    UpdateMatrix::Weight(target, value) =>{
                        
                        match self.props.seq_type  {
                            SequenceType::OffersOnly => {
                                self.props
                                    .root_matrix
                                    .borrow_mut()
                                    .children_groups
                                    .get_mut(0)
                                    .map(|s| {
                                        s.iter_mut()
                                            .find(|s| s.id() == target.id())
                                            .map(|s| {
                                                if let MatrixData::Offer(mut offer) =&s.value.data {
                                                    offer.weight=value;
                                                }
                                            })
                                    });
                            }
    
                            SequenceType::LandingPageAndOffers => {
                                self.props
                                    .root_matrix
                                    .borrow_mut()
                                    .children_groups
                                    .get_mut(0)
                                    .map(|s| {
                                        s.iter_mut()
                                            .find(|s| s.id() == target.id())
                                            .map(|s| {
                                                if let MatrixData::Offer(mut offer) =&s.value.data {
                                                    offer.weight=value;
                                                } else if let MatrixData::LandingPage(mut lp)=&s.value.data {
                                                    lp.weight=value;
                                                }
                                            })
                                    });
                            }
    
                            SequenceType::Matrix => {
                                if let Some(parent_node) = target.get_parent_node() {
                                    let found_parent_res = Matrix::search_next_depth(
                                        self.props
                                            .root_matrix
                                            .borrow_mut()
                                            .children_groups
                                            .iter_mut()
                                            .flatten(),
                                        parent_node.id.as_ref(),
                                        parent_node.depth,
                                    );
    
                                    match &found_parent_res {
                                        Ok(mut parent) =>{
                                            parent.children_groups.get_mut(target.group_idx()).map(|s| s.get_mut(target.item_idx()).map(|s| {
                                                
                                                match &mut s.value.data {
                                                    MatrixData::Offer(offer) =>{
                                                        offer.weight=value;
                                                    }
                                                    MatrixData::LandingPage(lp) =>{
                                                        lp.weight=value;
                                                    }
                                                    _=>{
                                                        notify_danger("Err: y656HG4G");
                                                    }
                                                }
                                            }));
                                        }
                                        Err(msg)=>{
                                            notify_danger(&msg);
                                            return false;
                                        }
                                    }
                                    
                                } else {
                                    notify_danger("No Parent Node");
                                    return false;
                                }
                            }
                        }
                    }
                    
                    UpdateMatrix::FillVoid(old, new) => {
                        match self.props.seq_type {
                            SequenceType::OffersOnly => {
                                self.props
                                    .root_matrix
                                    .borrow_mut()
                                    .children_groups
                                    .get_mut(0)
                                    .map(|s| {
                                        s.iter_mut()
                                            .find(|s| s.id() == old.id())
                                            .map(|s| s.transform_void(new))
                                    });
                            }
                            SequenceType::LandingPageAndOffers => {
                                self.props
                                    .root_matrix
                                    .borrow_mut()
                                    .children_groups
                                    .get_mut(old.group_idx())
                                    .map(|s| {
                                        s.iter_mut()
                                            .find(|s| s.id() == old.id())
                                            .map(|s| s.transform_void(new))
                                    });
                            }
                            SequenceType::Matrix => {
                                if let Some(parent_node) = old.get_parent_node() {
                                    let found_parent = Matrix::search_next_depth(
                                        self.props
                                            .root_matrix
                                            .borrow_mut()
                                            .children_groups
                                            .iter_mut()
                                            .flatten(),
                                        parent_node.id.as_ref(),
                                        parent_node.depth,
                                    );
                                    
                                    found_parent.map(|s| {
                                        s.children_groups.get_mut(old.group_idx()).map(|s| {
                                            s.get_mut(old.item_idx()).map(|s| s.transform_void(new))
                                        })
                                    });
                                } else {
                                    notify_danger("No Parent Node");
                                    return false;
                                    
                                    // self.props
                                    //     .root_matrix
                                    //     .borrow_mut()
                                    //     .children_groups
                                    //     .get_mut(old.group_idx())
                                    //     .map(|s| {
                                    //         s.iter_mut()
                                    //             .find(|s| s.id() == old.id())
                                    //             .map(|s| s.transform_void(new))
                                    //     });
                                }
                            }
                        }
                    }
                }
                self.props.transmit.emit(rc!(self.props.root_matrix));
            }

            Msg::RemoveChild(child_to_remove) => {
                match self.props.seq_type {
                    SequenceType::Matrix => self.remove_child(Some(child_to_remove), None, None),
                    SequenceType::OffersOnly => self.remove_child(None, None, Some(*child_to_remove)),
                    SequenceType::LandingPageAndOffers => self.remove_child(None, Some(*child_to_remove), None),
                }
                self.props.transmit.emit(rc!(self.props.root_matrix));
            }

            Msg::AddChild(void_child) => {

                match self.props.seq_type {
                    SequenceType::Matrix => self.remove_child(Some(void_child), None, None),
                    SequenceType::OffersOnly => self.remove_child(None, None, Some(*void_child)),
                    SequenceType::LandingPageAndOffers => {
                        self.remove_child(None, Some(*void_child), None)
                    }
                }
                self.props.transmit.emit(rc!(self.props.root_matrix));
            }

            Msg::Ignore => {}
            _ => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.local_matrix.value.eq(&self.props.local_matrix.value) {
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
            
            (SequenceType::OffersOnly, MatrixData::Source) => {
                let mut offer_children_nodes = VList::new();
                let offer_children = self.props.local_matrix.children_groups.get(0).unwrap();
                let source_local_matrix = rc!(self.props.local_matrix);
                let parent_matrix_value=source_local_matrix.value.parent_matrix.clone();
                let item_idx =source_local_matrix.new_item_idx(0).expect("g54%$gfse#");
                
                let add_callback = self
                    .link
                    .callback(move |_| Msg::AddChild(Rc::new(Matrix::void(parent_matrix_value, 0, item_idx, 1))));
                
                for matrix in offer_children {
                    offer_children_nodes.push(html!{
                        <MatrixBuilder
                        root_matrix=rc!(self.props.root_matrix)
                        local_matrix=rc!(matrix.clone())
                        state=rc!(self.props.state)
                        seq_type=SequenceType::OffersOnly
                        sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                        />
                    });
                }
    
                VNode::from(html! {
                        <tbody>
                            <tr>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=add_callback class="uk-button uk-button-large uk-button-primary">{"Add Offer"}</button></td>
                            </tr>
                            {offer_children_nodes}
                        </tbody>
                })
            }

            (SequenceType::OffersOnly, MatrixData::Void) => {
                let add_offer_callback =self.link.callback(|offer:Offer| Msg::UpdateMatrix(UpdateMatrix::FillVoid(rc!(self.props.local_matrix), Transform::Offer(offer))));
                let remove_callback= self.link.callback(Msg::RemoveChild(rc!(self.props.local_matrix)));
                
                VNode::from(html! {
                            <tr>
                                <td class="uk-text-truncate"><OfferDropdown state=rc!(state) eject=add_offer_callback /></td>
                                <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                            </tr>
                })
            }

            (SequenceType::OffersOnly, MatrixData::Offer(offer)) => {
                let blur_update_weight_callback =                     self.link.send_message(Msg::UpdateMatrix(UpdateMatrix::Weight(rc!(self.props.local_matrix), self.weight_buff)));
                let update_weight_callback= self.link.callback(Msg::UpdateWeight);
                let remove_callback= self.link.callback(Msg::RemoveChild(rc!(self.props.local_matrix)));
                let weight_val = if let MatrixData::Offer(o) = & self.props.local_matrix.value {
                    o.weight.to_string()
                } else {"".to_string()}
    
    
                VNode::from(html! {
                            <tr>
                                <td class="uk-text-truncate"><span>{format!("{}", &offer.name)}</span></td>
                                <td class="uk-text-truncate"><input class="uk-input" value=weight_val oninput=update_weight_callback onblur=blur_update_weight_callback placeholder="Weight" /></td>
                                <td class="uk-text-nowrap"><button onclick=remove_callback class="uk-button uk-button-small">{"Remove"}</button></td>
                            </tr>
                })
            }
    
            (SequenceType::LandingPageAndOffers, MatrixData::Source) =>{
                let mut nodes = VList::new();
                let fg_add_cb = self.link.callback(Msg::AddChild(Rc::new(Matrix::void(Some(arc!(self.props.local_matrix)), 0, self.props.local_matrix.new_item_idx(0).expect("54gsFr4fF"), 1))));
    
                nodes.push(html!{
                            <tr>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=fg_add_cb class="uk-button uk-button-large uk-button-primary">{"Add Element (Lander or Offer)"}</button></td>
                            </tr>
                });
                
                let first_group_nodes = self.props.local_matrix.children_groups.get(0).unwrap();
                
                for(item_idx, item) in first_group_nodes.iter().enumerate() {
    
                    nodes.push(html!{
                        <MatrixBuilder
                        root_matrix=rc!(self.props.root_matrix)
                        local_matrix=rc!(self.props.local_matrix)
                        state=rc!(self.props.state)
                        seq_type=SequenceType::LandingPageAndOffers
                        sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                        />
                });
            }
                nodes.push(html!{{divider!(2)}});
                
                let rest_of_groups = self.props.local_matrix.children_groups.iter().enumerate().filter(|(group_idx, _)| *group_idx != 0);
                
                for(group_idx,  group) in rest_of_groups {
                    let add_offer_to_group_cb=self.link.callback(Msg::AddChild(Rc::new(Matrix::void(Some(Arc::new(self.props.local_matrix.value.clone())), group_idx, group.len(), 1 ))));
                    nodes.push(html!{
                            <tr>
                                <td class="uk-text-nowrap uk-text-center">{"Offer Groups 2 Debug..."}</button></td>
                                <td class="uk-text-nowrap uk-text-center"><button onclick=add_offer_to_group_cb class="uk-button uk-button-large uk-button-primary">{"Add Offer"}</button></td>
                            </tr>
                });
                    
                    for (item_idx, item) in group.iter().enumerate() {
    
                        nodes.push(html!{
                        <MatrixBuilder
                        root_matrix=rc!(self.props.root_matrix)
                        local_matrix=rc!(self.props.local_matrix)
                        state=rc!(self.props.state)
                        seq_type=SequenceType::LandingPageAndOffers
                        sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                        />
                });
                    }
                }
                
                VNode::from(nodes)
            }
            
            (SequenceType::LandingPageAndOffers, matrix_data) =>{
                let rm_cb = self.link.callback(Msg::RemoveChild(rc!(self.props.local_matrix)));
                let oninput_update_weight_cb = self.link.callback(Msg::UpdateWeight);
                let onblur_update_weight_cb = self.link.callback(Msg::UpdateMatrix(UpdateMatrix::Weight(rc!(self.props.local_matrix), self.weight_buff)));
                let weight_value = self.weight_buff.to_string();
                
                match matrix_data {
                    MatrixData::LandingPage(lp) =>{
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
                    MatrixData::Offer(offer) =>{
                        
                        VNode::from(html! {
                            <tr>
                                <td class="uk-text-truncate">{format!("{}", &offer.name)}</td>
                                <td class="uk-text-nowrap">{"Offer"}</td>
                                <td class="uk-text-nowrap">{"NA"}</td>
                                <td class="uk-text-nowrap"><input type="number" oninput=oninput_update_weight_cb value=weight_value onblur=onblur_update_weight_cb class="uk-input" placeholder="Weight" /></td>
                                <td class="uk-text-nowrap"><button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button></td>
                            </tr>
                })
                    }
                    _=>{}
                }

            }
            
            _ => VNode::from(html! {}),
        }
    }
    
    // pub fn gen_offer_only_rows(&self, offer:&Matrix)->VNode{
    //     VNode::from(html! {
    //                     <MatrixBuilder
    //                     root_matrix=rc!(self.props.root_matrix)
    //                     local_matrix=local_matrix
    //                     state=rc!(self.props.state)
    //                     seq_type=SequenceType::OffersOnly
    //                     sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
    //                     />
    //                 }
    // }
    
    pub fn table_head(&self) -> VNode {
        match (self.props.seq_type, self.props.local_matrix.data()) {
            (SequenceType::OffersOnly, MatrixData::Source) => VNode::from(html! {
                <thead>
                    <tr>
                        <th class="uk-table-shrink uk-text-nowrap">{"Name"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Weight"}</th>
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
                        <th class="uk-table-shrink uk-text-nowrap">{"Weight"}</th>
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
                        <th class="uk-table-shrink uk-text-nowrap">{"Weight"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"CTA's"}</th>
                        <th class="uk-table-shrink uk-text-nowrap">{"Remove"}</th>
                    </tr>
                </thead>
            }),

            _ => VNode::from(html! {}),
        }
    }

    // pub fn weight_header(&self) -> VNode {
    //     if self.props.local_matrix.has_children_in_groups() {
    //         VNode::from(html! {<th class="uk-table-shrink uk-text-nowrap">{"Weight"}</th>})
    //     } else {
    //         html! {}
    //     }
    // }

    pub fn add_child(
        &self,
        matrix_child_target_to_add: Option<Rc<Matrix>>,
        landing_page_source_child_target_to_add: Option<Matrix>,
        offer_source_child_target_to_add: Option<Matrix>,
    ) -> Result<usize, String> {
        match self.props.seq_type {
            SequenceType::Matrix => self.add_child_to_matrix(matrix_child_target_to_add?),

            SequenceType::LandingPageAndOffers => {
                self.add_landing_page_child_to_source(landing_page_source_child_target_to_add?)
            }

            SequenceType::OffersOnly => {
                self.add_child_to_offer_source(offer_source_child_target_to_add?)
            }
        }
    }

    pub fn add_child_to_matrix(&self, child_target: Rc<Matrix>) -> Result<usize, String> {
        if let Some(parent_node) = child_target.get_parent_node() {
            let target_parent_depth = parent_node.depth;
            let target_parent_id = parent_node.id.as_ref();
            let child_group_index = child_target.group_idx();
            let child_item_index = child_target.item_idx();
            let mut root = self.props.root_matrix.borrow_mut();

            let found_parent = Matrix::search_next_depth(
                root.children_groups.iter_mut().flatten(),
                target_parent_id,
                target_parent_depth,
            );

            match found_parent {
                Ok(parent_target) => {
                    if let Some(target_group) =
                        parent_target.children_groups.get_mut(child_group_index)
                    {
                        let bf = target_group.len();
                        target_group.insert(child_item_index, *child_target);
                        parent_target.synchronize_matrix_child_groups();
                        Ok(target_group.len() - bf)
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
                let bf = group.len();
                group.insert(child_item_index, *child_target);
                Ok(group.len() - bf)
            } else {
                Err("Invalid group idx".to_string())
            }
        }
    }

    pub fn add_landing_page_child_to_source(
        &self,
        landing_page_source_child_target: Matrix,
    ) -> Result<usize, String> {
        let group_idx = landing_page_source_child_target.group_idx();
        let item_idx = landing_page_source_child_target.item_idx();

        let mut source = self.props.root_matrix.borrow_mut();
        if let Some(group) = source.children_groups.get_mut(group_idx) {
            let bf = group.len();
            group.insert(item_idx, landing_page_source_child_target);
            source.root_synchronize_landing_page_child_groups();
            Ok(group.len() - bf)
        } else {
            Err(new_string!("group not found"))
        }
    }

    pub fn add_child_to_offer_source(
        &self,
        offer_source_child_target: Matrix,
    ) -> Result<usize, String> {
        let mut source = self.props.root_matrix.borrow_mut();
        let item_index = offer_source_child_target.item_idx();

        if let Some(g) = source.children_groups.get_mut(0) {
            let bf = g.len();
            g.insert(item_index, offer_source_child_target);
            Ok(g.len() - bf)
        } else {
            Err(new_string!("Group Not Found"))
        }
    }

    pub fn remove_child(
        &self,
        matrix_child_target: Option<Rc<Matrix>>,
        landing_page_source_child_target: Option<Matrix>,
        offer_source_child_target: Option<Matrix>,
    ) -> Result<Matrix, String> {
        match self.props.seq_type {
            SequenceType::Matrix => self.remove_child_for_matrix(matrix_child_target?),

            SequenceType::LandingPageAndOffers => {
                self.remove_landing_page_child_from_source(landing_page_source_child_target?)
            }

            SequenceType::OffersOnly => {
                self.remove_offer_child_from_source(offer_source_child_target?)
            }
        }
    }

    pub fn remove_landing_page_child_from_source(
        &self,
        landing_page_source_child_target: Matrix,
    ) -> Result<Matrix, String> {
        let group_idx = landing_page_source_child_target.group_idx();
        let item_idx = landing_page_source_child_target.item_idx();

        let mut source = self.props.root_matrix.borrow_mut();
        if let Some(group) = source.children_groups.get_mut(group_idx) {
            if let Some(i) = group.get(item_idx) {
                let res = group.remove(item_idx);
                source.root_synchronize_landing_page_child_groups();
                Ok(res)
            } else {
                Err(new_string!("item not found"))
            }
        } else {
            Err(new_string!("group not found"))
        }
    }

    pub fn remove_offer_child_from_source(
        &self,
        offer_source_child_target: Matrix,
    ) -> Result<Matrix, String> {
        let mut source = self.props.root_matrix.borrow_mut();
        let item_index = offer_source_child_target.item_idx();
        if let Some(g) = source.children_groups.get_mut(0) {
            if let Some(i) = g.get(item_index) {
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
            let target_parent_depth = parent_node.depth;
            let target_parent_id = parent_node.id.as_ref();
            let child_group_index = child_target.group_idx();
            let child_item_index = child_target.item_idx();
            let mut root = self.props.root_matrix.borrow_mut();

            let found_parent = Matrix::search_next_depth(
                root.children_groups.iter_mut().flatten(),
                target_parent_id,
                target_parent_depth,
            );

            match found_parent {
                Ok(parent_target) => {
                    if let Some(target_group) =
                        parent_target.children_groups.get_mut(child_group_index)
                    {
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
