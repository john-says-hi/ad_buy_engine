use yew::prelude::*;

pub enum Msg {
    OnInput(InputData),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub tooltip: String,
}

pub struct TextInput {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnInput(text) => {
                self.props.oninput.emit(text);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let label = if self.props.label.is_empty() {
            html! {}
        } else {
            label!(&self.props.label)
        };

        html! {
                    <div class="uk-margin">
                        {label}
                        <input class="uk-input" type="text" placeholder=self.props.placeholder oninput=self.link.callback(Msg::OnInput) value=self.props.value />
                    </div>
        }
    }
}
