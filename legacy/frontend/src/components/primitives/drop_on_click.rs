use yew::prelude::*;

pub struct DropOnClick {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub node: Html,
}

impl Component for DropOnClick {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.props.node = self.props.node.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div uk-drop="pos: bottom-justify;mode: click;">
                <div class="uk-card uk-card-body uk-card-default" style="border:2px solid blue;">
                    {self.props.node.clone()}
                </div>
            </div>
        }
    }
}
