use yew::prelude::*;

mod components;
mod data;

pub mod utils;

use components::{footer::Footer, info::Info, portfolio::Portfolio, theme::Theme};
use data::PROJECT_LIST;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="main-container">
                <Info />
                <Portfolio list={&PROJECT_LIST} />
                <Theme />
            </div>
            <footer>
                <Footer />
            </footer>
        </>
    }
}

// entry
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
