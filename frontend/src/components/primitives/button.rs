use yew::prelude::*;

pub enum Msg {
    Click,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub onclick: Callback<()>,
    #[prop_or(false)]
    pub is_disabled: bool,
    #[prop_or_default]
    pub class: String,
}

pub struct Button {
    link: ComponentLink<Self>,
    text: String,
    props: Props,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: props.text.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.props.onclick.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let text = &self.text;

        let mut class = format!(
            " uk-button uk-button-primary  uk-button-small {}",
            &self.props.class
        );

        if self.props.is_disabled {
            class.push_str(" uk-disabled")
        };

        html! {
        <div><button
        class=class
        onclick=self.link.callback(|_| Msg::Click)
        >
        {text}
        </button></div>
        }
    }
}
