use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct HeadlineComp {
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
}

impl Component for HeadlineComp {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HeadlineComp { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let text = &self.props.text;

        html! {
            <h2>
                {text}
            </h2>
        }
    }
}
