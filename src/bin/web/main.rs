use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <h1>{env!("CARGO_PKG_NAME")}</h1>
    }
}
