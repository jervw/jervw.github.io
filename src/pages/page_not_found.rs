use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-not-found">
                    <h1>
                        { "404" }
                    </h1>
                    <h2>
                        { "Page page does not exist" }
                    </h2>
            </div>
        }
    }
}
