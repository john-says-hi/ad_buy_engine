#[macro_export]
macro_rules! callback {
    ($name:ident, $function:expr) => {{
        $name.link.callback($function)
    }};
}

#[macro_export]
macro_rules! label {
    ($label:expr) => {html!{<span class="uk-label uk-label-large uk-label-primary">{$label}</span>}};
    ("g", $label:expr) => {html!{<span class="uk-label uk-label-large uk-label-success" >{$label}</span>}};
    ("o", $label:expr) => {html!{<span class="uk-label uk-label-large uk-label-warning" >{$label}</span>}};
    ("gb", $label:expr) => {html!{<span class="uk-label uk-label-large uk-label-success" style="color: #000000" >{$label}</span>}};
    ("m", $label:expr) => {html!{<span class="uk-label uk-label uk-label-primary">{$label}</span>}};
    ("s", $label:expr) => {html!{<span class="uk-label uk-label-small uk-label-primary">{$label}</span>}};
    ("p", $label:expr) => {html!{<span class="uk-label uk-label-small uk-label-purple">{$label}</span>}};
}

#[macro_export]
macro_rules! divider {
    () => {
        html! {<hr class="uk-divider-small" />}
    };
    (1) => {
        html! {<hr class="uk-divider-small" />}
    };
    (2) => {
        html! {<hr class="uk-divider-icon" />}
    };
}

#[macro_export]
macro_rules! state_clone {
    ($loc:expr) => {
        Rc::clone(&$loc)
    };
}

#[macro_export]
macro_rules! tab_is_active {
    ($type_to_match:pat, $active_route:expr) => {
        if let $type_to_match = $active_route {
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! dropdown_is_active {
    ($($arm_to_match:pat)+, $active_route:expr) =>{
        match $active_route {
            $(
            $arm_to_match => true,
            )+
            _=>false,
        }
    }
}

#[macro_export]
macro_rules! border {
    ($color:expr) => {
        format!("border: 2px solid {};", $color);
    };
}

#[macro_export]
macro_rules! wrap {
    ($data:expr) => {{
        Rc::new(RefCell::new($data))
    }};
}

#[macro_export]
macro_rules! rc {
    ($data:expr) => {{
        Rc::clone(&$data)
    }};
}
