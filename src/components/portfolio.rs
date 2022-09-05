use yew::prelude::*;

pub struct Portfolio;

impl Component for Portfolio {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="portfolio-container">
                <h1> { "Portfolio" } </h1>
                <div class="tagline">{ "A Selection of my Best Works" } </div>

                <div class="portfolio">
                    <fieldset class="portfolio-item">
                        <legend> { "urbaani-cli" } </legend>
                            <div>
                                { "word dictionary on your terminal" }
                            </div>
                            <div>
                                <i class="fas fa-hammer"></i>
                                {"Rust"}
                            </div>
                            <div>
                                <i class="fas fa-code-branch"></i>
                                <a href="https://github.com/jervw/urbaani-cli">
                                    { "jervw/urbaani-cli" }
                                </a>
                            </div>
                    </fieldset>

                    <fieldset class="portfolio-item">
                        <legend> { "chess-engine" } </legend>
                            <div>
                                { "chess program using Negamax Alpha-Beta Pruning" }
                            </div>
                            <div>
                                <i class="fas fa-hammer"></i>
                                {"C++"}
                            </div>
                            <div>
                                <i class="fas fa-code-branch"></i>
                                <a href="https://github.com/jervw/chess-engine">
                                    { "jervw/chess-engine" }
                                </a>
                            </div>
                    </fieldset>
                </div>
            </div>
        }
    }
}
