use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="footer-container">
            <div class="footer-left">
                <div><i class="fas fa-copyright"></i>{ "2022 Jere Vuola. " } <br class="footer-break"/>{ "All rights reserved." }</div>
                <div>{ "Powered by Yew" }</div>
            </div>
            <div class="footer-right">
                <a href="https://github.com/jervw/website">
                    <i class="fab fa-github"></i> { "Source Code" }
                </a>
            </div>
            </div>
        }
    }
}
