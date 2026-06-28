use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct SearchInput {
    pub props: Props,
    pub link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oninput: Callback<String>,
}

pub enum Msg {
    OnInput(String),
}

impl Component for SearchInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SearchInput { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnInput(text) => {
                self.props.oninput.emit(text);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|id: InputData| Msg::OnInput(id.value));

        html! {
        <div>
            <form class="uk-search uk-search-default">
                <span uk-search-icon=""></span>
                <input class="uk-search-input" type="search" placeholder="Search..." oninput=callback />
            </form>
        </div>
        }
    }
}
