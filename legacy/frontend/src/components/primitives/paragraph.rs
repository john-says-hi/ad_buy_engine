use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct ParagraphComp {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
}

impl Component for ParagraphComp {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ParagraphComp { props }
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
            <p>
                {text}
            </p>
        }
    }
}
