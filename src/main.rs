use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{about::About,turbofish::Turbofish};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Turbofish,

    #[at("/:path")]
    TurbofishWithGuts {path: String},

    #[at("/about")]
    About,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Turbofish => html! {<Turbofish/>},
        Route::TurbofishWithGuts{path} => html! {
            <Turbofish path={urlencoding::decode(&path).unwrap_or_default().into_owned()}/>
        },
        Route::About => html! {<About/>},
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer
        ::<App> // Behold, the mighty Turbofish! Truly a magnificent creature.
    ::new().render();
}
