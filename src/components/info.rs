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
            <div class="info-container">
            <h1>{ "Jere Vuola" }</h1>
                <div class="tagline"> { "Software Engineer" } </div>
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
                        <li>
                            <i class="fas fa-briefcase"></i>
                            { "AFRY" }
                        </li>
                    </ul>
                </fieldset>

                <fieldset>
                    <legend> { "Skills" } </legend>
                    <ul>
                        <li>
                            <i class="fas fa-code"></i>
                            { "Rust, C++, Python, C#" }
                        </li>
                        <li>
                            <i class="fas fa-desktop"></i>
                            { "Git, Linux, Docker, Kubernetes" }
                        </li>
                    </ul>
                </fieldset>
                <fieldset>
                    <legend> { "Links" } </legend>
                    <ul class="links">
                        <li>
                            <i class="fab fa-github-square"></i>
                            <a href="https://github.com/jervw">
                            { "Github" }
                            </a>
                        </li>
                        <li>
                            <i class="fab fa-linkedin"></i>
                            <a href="https://www.linkedin.com/in/jerevuola">
                            { "LinkedIn" }
                            </a>
                        </li>
                        <li>
                            <i class="fas fa-envelope-square"></i>
                            <a href="mailto:vuolajere@gmail.com">
                            { "vuolajere@gmail.com" }
                            </a>
                        </li>
                        <li>
                            <i class="fas fa-key"></i>
                            <a href="https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x2a19308ba17f69683bb4ff821e3fce4bf382e951">
                                { "GPG" }
                            </a>
                        </li>
                    </ul>
                </fieldset>
            </div>
        }
    }
}
