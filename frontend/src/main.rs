use yew_router::prelude::*;
use yew::prelude::*;
use gloo_net::http::Request;

use common::TestStruct;

mod components;

use components::navbar::Navbar;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let test: UseStateHandle<Option<TestStruct>> = use_state(|| None);
    {
        let test = test.clone();
        use_effect_with_deps(move |_| {
            let test = test.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_test: TestStruct = Request::get("http://localhost:8081/test")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                test.set(Some(fetched_test))
            });
            || ()
        }, ());
    }

    html! {
        <div>
            if let Some(x) = &*test {
                <b>{"test"}</b>
                { &x.name }
            }
            <h1>{ "Home!!" }</h1>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home />  },
        Route::Secure => html! { <Secure /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />

            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}