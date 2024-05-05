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
                        // TODO Go get a job :p
                        // <li>
                        //     <i class="fas fa-briefcase"></i>
                        //     { "" }
                        // </li>
                    </ul>
                </fieldset>

                <fieldset>
                    <legend> { "Skills" } </legend>
                    <ul>
                        <li>
                            <i class="fas fa-code"></i>
                            { "C++, Rust, Python, Nix" }
                        </li>
                        <li>
                            <i class="fas fa-desktop"></i>
                            { "Git, Linux, Docker, Ansible" }
                        </li>
                    </ul>
                </fieldset>
                <fieldset>
                    <legend> { "Links" } </legend>
                    <ul class="links">
                        <li>
                            <i class="fab fa-github-square"></i>
                            <a href="https://github.com/jervw">
                            { "GitHub" }
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
                            <a href="mailto:jervw@pm.me">
                            { "jervw@pm.me" }
                            </a>
                        </li>
                        <li>
                            <i class="fas fa-key"></i>
                            <a href="https://keys.openpgp.org/vks/v1/by-fingerprint/56C25B5B20756352B4B0E17EF188371747DA5895">
                                { "GPG" }
                            </a>
                        </li>
                    </ul>
                </fieldset>
            </div>
        }
    }
}
