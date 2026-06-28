use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct SectionComp {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
}

impl Component for SectionComp {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SectionComp { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let node = self.props.node.clone();

        html! {
            <div class="uk-width-1-1 uk-section uk-padding-small">
                {node}
            </div>
        }
    }
}
