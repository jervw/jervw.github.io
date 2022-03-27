use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header>
            <h1>{ "Hi! 👋 My name is Jere.
            I'm a game developer student" }</h1>
            </header>
        }
    }
}
