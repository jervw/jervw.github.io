use yew::prelude::*;

pub struct Info;

impl Component for Info {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="info">
                <h1>{ "Jere Vuola" }</h1>
                <div class="tagline">{ "Software Engineer" }</div>
                <fieldset>
                    <legend>{ "About me" }</legend>
                    <ul>
                        <li>
                            <i class="fas fa-map-marker-alt"></i>
                            { "Espoo, Finland" }
                        </li>
                        <li>
                            <i class="fas fa-university"></i>
                            { "Metropolia UAS" }
                        </li>
                    </ul>
                </fieldset>

                <fieldset>
                    <legend>{ "Skills" }</legend>
                    <ul>
                        <li>
                            <i class="fas fa-code"></i>
                            { "Rust, Python, C++, C#" }
                        </li>
                        <li>
                            <i class="fas fa-desktop"></i>
                            { "Git, Neovim, Docker, Linux" }
                        </li>
                    </ul>
                </fieldset>
                <fieldset>
                    <legend>{ "Links" }</legend>
                    <ul class="links">
                        <li>
                            <i class="fab fa-github-square"></i>
                            <a href="https://github.com/jervw"> 
                            { "Github" } 
                            </a>
                        </li>
                        <li>
                            <i class="fab fa-linkedin"></i>
                            <a href="https://www.linkedin.com/..todo">
                            { "LinkedIn" }
                            </a>
                        </li>
                        <li>
                            <i class="fas fa-envelope-square"></i>
                            <a href="mailto:vuolajere@gmail.com">
                            { "vuolajere@gmail.com" }
                            </a>
                        </li>
                    </ul>
                </fieldset>
            </div>
        }
    }
}
