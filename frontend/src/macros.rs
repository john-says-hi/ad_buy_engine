#[macro_export]
macro_rules! callback {
    ($name:ident, $function:expr) => {
    {
        $name.link.callback($function)
    }
    }
}

#[macro_export]
macro_rules! state_clone {
    ($loc:expr) => {Rc::clone(&$loc)}
}

#[macro_export]
macro_rules! tab_is_active {
    ($type_to_match:pat, $state:expr) =>{
        if let $type_to_match = $state.borrow().return_app_route() {
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! dropdown_is_active {
    ($($arm_to_match:pat)+, $state:expr) =>{
        match $state.borrow().return_app_route() {
            $(
            $arm_to_match => true,
            )+
            _=>false,
        }
    }
}