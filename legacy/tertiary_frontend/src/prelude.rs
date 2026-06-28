use yew::services::DialogService;

pub fn alert(msg: &str) {
    DialogService::alert(msg)
}
