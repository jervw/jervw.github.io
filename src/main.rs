use yew::prelude::*;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod components;
use components::{footer::Footer, info::Info};

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
            <div class="container">
                <Info />
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
