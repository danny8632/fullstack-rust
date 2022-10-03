use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub enum Msg {
    Toggle,
    Reset,
}

#[derive(Eq, PartialEq, Properties)]
pub struct Props;

pub struct Navbar {
    navbar_toggled: bool,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_toggled: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.navbar_toggled = !self.navbar_toggled;
                true
            },
            Msg::Reset => {
                self.navbar_toggled = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = if self.navbar_toggled { "responsive" } else { "" };

        let onclick = ctx.link().callback(|_| Msg::Toggle);

        html! {
            <div class={classes!("topnav", classes)}>
                <Link<Route> classes={classes!("link", "active")} to={Route::Home}>
                    { "Home" }
                </Link<Route>>

                <Link<Route> classes={"link"} to={Route::Secure}>
                    { "Secure" }
                </Link<Route>>

                <a href="javascript:void(0);" class="icon" {onclick}>
                    <i class="fa fa-bars"></i>
                </a>
            </div>
        }
    }
}