use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct GridComp {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
}

impl Component for GridComp {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GridComp { props }
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
            <div uk-grid="">
                {node}
            </div>
        }
    }
}
