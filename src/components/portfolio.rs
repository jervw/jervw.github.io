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
                        <legend> { "repo1" } </legend>
                            <div>
                                { "Repository description" }
                            </div>
                            <div>
                                <i class="fas fa-hammer"></i>
                                {"Rust"}
                            </div>
                            <div>
                                <i class="fas fa-code-branch"></i>
                                <a href="https://github.com/jervw/repo1">
                                    { "jervw/repo1" }
                                </a>
                            </div>
                    </fieldset>

                    <fieldset class="portfolio-item">
                        <legend> { "repo2" } </legend>
                            <div>
                                { "Repository description" }
                            </div>
                            <div>
                                <i class="fas fa-hammer"></i>
                                {"Rust"}
                            </div>
                            <div>
                                <i class="fas fa-code-branch"></i>
                                <a href="https://github.com/jervw/repo1">
                                    { "jervw/repo1" }
                                </a>
                            </div>
                    </fieldset>
                    
                    <fieldset class="portfolio-item">
                        <legend> { "repo3" } </legend>
                            <div>
                                { "Repository description" }
                            </div>
                            <div>
                                <i class="fas fa-hammer"></i>
                                {"Rust"}
                            </div>
                            <div>
                                <i class="fas fa-code-branch"></i>
                                <a href="https://github.com/jervw/repo1">
                                    { "jervw/repo1" }
                                </a>
                            </div>
                    </fieldset>
                </div>
            </div>
        }
    }
}
