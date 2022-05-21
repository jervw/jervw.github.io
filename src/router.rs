use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
    #[to = "/start"]
    Start,
}

pub struct Router;

impl Component for Router {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|switch: Routes| match switch {
            Routes::Index | Routes::Home => html! {
                <pages::Home/>
            },
        });

        html! {
            <Router<Routes, ()>
                render = render_func
            />
        }
    }
}
