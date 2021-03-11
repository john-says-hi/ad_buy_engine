use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};
use yewtil::NeqAssign;

pub struct Modal {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
    pub id: String,
    pub title: String,
    pub onsubmit: Callback<()>,
    pub onexit: Callback<()>,
    #[prop_or("esc-close:false;bg-close:false;".to_string())]
    pub options: String,
}

pub enum Msg {
    Submit,
    Exit,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Modal { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Exit => self.props.onexit.emit(()),
            Msg::Submit => self.props.onsubmit.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let node = self.props.node.clone();
        let onsubmit = self.link.callback(|_| Msg::Submit);
        let onexit = self.link.callback(|_| Msg::Exit);

        html! {
            <div id=&self.props.id uk-modal={&self.props.options}>
                <div class="uk-modal-dialog">
                    <button class="uk-modal-close-default" type="button" uk-close="" onclick=onexit.clone()></button>
                    <div class="uk-modal-header">
                        <h2 class="uk-modal-title">{&self.props.title}</h2>
                    </div>
                    <div class="uk-modal-body">{self.props.node.clone()}</div>
                    <div class="uk-modal-footer">
                        <button class="uk-button uk-button-default uk-modal-close" type="button" onclick=onexit>{"Cancel"}</button>
                        <button class="uk-button uk-button-primary" type="button" onclick=onsubmit>{"Save"}</button>
                    </div>
                </div>
            </div>
        }
    }
}
