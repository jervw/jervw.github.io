use yew::prelude::*;

use crate::data::Project;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub project: &'static Project,
}
pub struct ProjectCard;

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let project = &ctx.props().project;
        html! {
            <fieldset class="portfolio-item">
                <legend> { project.name } </legend>
                    <div>
                        { project.description }
                    </div>
                    <div>
                        <i class="fas fa-hammer"></i>
                        { project.languages.join(", ") }
                    </div>
                    <div>
                        <i class="fas fa-code-branch"></i>
                        <a href= { project.url } >
                            { format!("{}/{}", project.owner, project.repo_name) }
                        </a>
                    </div>
            </fieldset>
        }
    }
}
