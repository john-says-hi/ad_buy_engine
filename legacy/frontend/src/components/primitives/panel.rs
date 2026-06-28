use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct Panel {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
    pub text: String,
    #[prop_or_default]
    pub style: String,
}

impl Component for Panel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Panel { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let text = self.props.text.clone();

        html! {
            <div class="uk-panel" style=&self.props.style>
                <h3 class="uk-panel-title">{&self.props.text}</h3>
                {self.props.node.clone()}
            </div>
        }
    }
}
