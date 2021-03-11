use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct Card {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Card { props }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let node = self.props.node.clone();

        html! {
            <div class=" uk-card uk-card-default uk-card-body uk-border-rounded" >
                {node}
            </div>
        }
    }
}
