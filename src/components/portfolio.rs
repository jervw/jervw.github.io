use yew::prelude::*;

use crate::components::project_card::ProjectCard;
use crate::data::ProjectList;

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub list: &'static ProjectList,
}

pub struct Portfolio;

impl Component for Portfolio {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_content: Html = ctx
            .props()
            .list
            .iter()
            .map(|i| {
                html! {
                    <ProjectCard project={i} />
                }
            })
            .collect();

        html! {
            <div class="portfolio-container">
                <h1> { "Portfolio" } </h1>
                <div class="tagline">{ "A Selection of my Best Works" } </div>

                <div class="portfolio">
                    {list_content}
                </div>
            </div>
        }
    }
}
