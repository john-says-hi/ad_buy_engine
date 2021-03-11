use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub label: String,
    pub callback: Callback<bool>,
}

pub struct CheckBox {
    props: Props,
    link: ComponentLink<Self>,
    checked: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for CheckBox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked;
        Self {
            props,
            link,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.checked = !self.checked;
                self.props.callback.emit(self.checked)
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(|_| Msg::Toggle);

        if self.checked {
            html! {
            <div class="uk-margin uk-grid-small uk-child-width-auto uk-grid">
                <label><input class="uk-checkbox" type="checkbox" checked=true oninput=cb />{&self.props.label}</label>
            </div>
            }
        } else {
            html! {
            <div class="uk-margin uk-grid-small uk-child-width-auto uk-grid">
                <label><input class="uk-checkbox" type="checkbox" oninput=cb />{&self.props.label}</label>
            </div>
            }
        }
    }
}
