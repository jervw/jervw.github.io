use yew::prelude::*;

mod components;
use components::{footer::Footer, info::Info, portfolio::Portfolio};

pub enum ColorTheme {
    Latte,
    Mocha,
}

#[derive(PartialEq, Debug)]
struct ThemeState {
    current: &'static str,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="main-container">
                <Info />
                <Portfolio />
            </div>
            <footer>
                <Footer />
            </footer>
        </>
    }
}

// entry
fn main() {
    yew::start_app::<App>();
}
