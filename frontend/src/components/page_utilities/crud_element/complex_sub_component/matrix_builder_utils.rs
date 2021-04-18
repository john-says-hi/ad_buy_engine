use super::matrix_builder::*;
use crate::notify_danger;
use ad_buy_engine::constant::{COLOR_BLUE, COLOR_GRAY, MONEY_GREEN};
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::matrix::Matrix;
use ad_buy_engine::data::elements::offer::Offer;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

impl MatrixBuilder {
    pub fn matrix_lander_base(&self, lander: &LandingPage) -> VNode {
        // let children = if self.children_hidden {
        //     html! {}
        // } else {
        //     html! {
        //         <>
        //             {divider!(2)}
        //             {self.entry_point_cta_groups()}
        //         </>
        //     }
        // };

        VNode::from(html! {
        <div class="uk-overflow-auto uk-card uk-card-default uk-card-body">
            <div class="uk-grid-column-small uk-grid-row-small uk-child-width-1-1" uk-grid="">
                {self.matrix_lander_row(lander)}
                    // {divider!(2)}
                    {self.entry_point_cta_groups()}
            </div>
        </div>
            })
    }

    pub fn matrix_lander_row(&self, lander: &LandingPage) -> VNode {
        let oi_weight_cb = self.link.callback(move |i: InputData| Msg::UpdateWeight(i));
        let ob_weight_cb = self
            .link
            .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Weight));
        let rm_cb = self
            .link
            .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));
        let depth = self.props.local_matrix.read().expect("%GSDF").depth();
        let depth_border = format!("border-bottom: 2px solid {}", color_depth_border(depth));

        VNode::from(html! {
        <div style=depth_border class="uk-grid-column-small uk-grid-row-small uk-child-width-auto uk-no-wrap uk-text-center" uk-grid="">

            <div uk-tooltip="title:Click to Toggle Matrix Children as Visible or Not">
                <span class="fa fa-eye"></span>
            </div>

            <div>
                <span class="fa fa-door-open" style=format!("color:{};", color_depth_border(depth))></span><span>{format!(" {} CTAs - Depth {} Lander:", lander.number_of_calls_to_action, depth)}</span>
            </div>

            <div class="uk-text-truncate" uk-tooltip="title:Name">
                {&lander.name}
            </div>

            <div uk-tooltip="title:Weight">
                <input type="number" oninput=oi_weight_cb value=format!("{}",&lander.weight) onblur=ob_weight_cb class="uk-input uk-form-width-small uk-form-small" placeholder="Weight" />
            </div>

            <div>
                <button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button>
            </div>
        </div>
            })
    }

    pub fn matrix_offer_row(&self, offer: &Offer) -> VNode {
        let oi_weight_cb = self.link.callback(move |i: InputData| Msg::UpdateWeight(i));
        let ob_weight_cb = self
            .link
            .callback(|_| Msg::UpdateMatrix(UpdateMatrix::Weight));
        let rm_cb = self
            .link
            .callback(move |_| Msg::UpdateMatrix(UpdateMatrix::Remove));
        let depth = self.props.local_matrix.read().expect("%GSDF").depth();
        // let depth_border = format!(
        //     "border-left-style:solid;border-left-color:{};",
        //     color_depth_border(depth)
        // );

        VNode::from(html! {
        <div class="uk-grid-column-small uk-grid-row-small uk-child-width-auto uk-no-wrap uk-text-center" uk-grid="">
            <div>
                <span class="fa fa-money-bill-alt" style=format!("color:{};", color_depth_border(depth))><span>{format!(" Depth {} Offer:", depth)}</span></span>
            </div>

            <div class="uk-text-truncate" uk-tooltip="title:Name;">
                {&offer.name}
            </div>
            <div uk-tooltip="title:Weight;">
                <input type="number" oninput=oi_weight_cb value=format!("{}",&offer.weight) onblur=ob_weight_cb class="uk-input uk-form-width-small uk-form-small" placeholder="Weight" />
            </div>

            <div>
                <button onclick=rm_cb class="uk-button uk-button-small">{"Remove"}</button>
            </div>
        </div>
            })
    }

    pub fn cta_group(
        &self,
        total_groups: usize,
        group_idx: usize,
        group: &Vec<Arc<RwLock<Matrix>>>,
    ) -> VNode {
        let group_style = format!("border-style: dotted;");
        let depth = self.props.local_matrix.read().unwrap().value.depth;
        // let style= format!("  border-style: dashed;border-width: 2px;")
        let style = format!("border:2px dashed {}", color_depth_border(depth));
        let add_cb = callback!(self, move |_| Msg::UpdateMatrix(UpdateMatrix::Add(
            group_idx
        )));

        let mut items = VList::new();
        for (idx, item) in group.iter().enumerate() {
            items.push(html! {
                                <MatrixBuilder
                                root_matrix=arc!(self.props.root_matrix)
                                local_matrix=arc!(item)
                                state=rc!(self.props.state)
                                seq_type=SequenceType::Matrix
                                sequence_builder_link=Rc::clone(&self.props.sequence_builder_link)
                                />
            })
        }

        VNode::from(html! {
        <div class="uk-margin" style=style >
            {label!(format!("CTA Group {} of {}", &group_idx + 1, total_groups))} <button onclick=add_cb class="uk-button uk-button-small uk-margin-left-large">{"Add Variant"}</button>
            <div>
                {items}
            </div>
        </div>
        })
    }

    pub fn entry_point_cta_groups(&self) -> VNode {
        let mut group_nodes = VList::new();
        let total_groups = self
            .props
            .local_matrix
            .read()
            .unwrap()
            .children_groups
            .len();

        for (group_idx, group) in self
            .props
            .local_matrix
            .read()
            .unwrap()
            .children_groups
            .iter()
            .enumerate()
        {
            group_nodes.push(self.cta_group(total_groups, group_idx, group))
        }

        VNode::from(group_nodes)
    }

    pub fn matrix_lander_header(&self) -> VNode {
        VNode::from(html! {
            <div class="uk-grid-column-small uk-grid-row-small uk-child-width-1-6 uk-no-wrap uk-text-center" uk-grid="">
                <div>
                    {"Depth"}
                </div>
                <div>
                    {"Type"} // ENTRY / EXIT
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
        })
    }

    pub fn holder(&self) -> VNode {
        VNode::from(html! {})
    }
}
