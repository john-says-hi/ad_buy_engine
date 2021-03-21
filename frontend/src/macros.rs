#[macro_export]
macro_rules! callback {
    ($name:ident, $function:expr) => {
    {
        $name.link.callback($function)
    }
    }
}

#[macro_export]
macro_rules! label {
    ($label:expr) => {html!{<span class="uk-label uk-label-large uk-label-primary">{$label}</span>}}
}

#[macro_export]
macro_rules! divider {
    () => {html!{<hr class="uk-divider-small" />}}
}

#[macro_export]
macro_rules! state_clone {
    ($loc:expr) => {Rc::clone(&$loc)}
}

#[macro_export]
macro_rules! tab_is_active {
    ($type_to_match:pat, $active_route:expr) =>{
        if let $type_to_match = $active_route {
            true
        } else {
            false
        }
    }
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