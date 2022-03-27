use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{home::Home, page_not_found::PageNotFound};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <p>{ "Powered by Yew" }</p>
                </footer>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

// entry
fn main() {
    yew::start_app::<App>();
}
