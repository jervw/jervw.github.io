use yew::prelude::*;

pub struct Start;

impl Component for Start {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-not-found">
                    <h1>
                        { "Start" }
                    </h1>

            </div>
        }
    }
}
