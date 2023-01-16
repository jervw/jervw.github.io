use crate::utils;
use yew::html;
use yew::prelude::*;

pub enum Msg {
    ToggleTheme,
}

pub struct Theme {
    dark_mode: bool,
}

const LOCAL_STORAGE_KEY: &str = "dark_mode";

impl Component for Theme {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // try to get the theme from local storage
        let dark_mode = utils::local_storage_get(LOCAL_STORAGE_KEY)
            .map(|s| s == "true")
            .unwrap_or({
                // if not found, try use the system theme
                let media_query = utils::media_query("(prefers-color-scheme: dark)");
                media_query.map(|mq| mq.matches()).unwrap_or(false)
            });

        Self { dark_mode }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                utils::local_storage_set(LOCAL_STORAGE_KEY, &self.dark_mode.to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let body = utils::body();

        if self.dark_mode {
            body.set_class_name("dark");
        } else {
            body.set_class_name("light");
        }

        html! {
            <div class="theme-switch">
                <button class="theme-button" onclick={link.callback(|_| Msg::ToggleTheme)}>
                    <i class={if self.dark_mode {
                        "fas fa-moon"
                    } else {
                        "fas fa-sun"
                    }}></i>
                </button>
            </div>
        }
    }
}
