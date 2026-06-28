#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<admin_dashboard::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
