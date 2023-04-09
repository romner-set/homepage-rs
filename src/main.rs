use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{about::About,turbofish::Turbofish};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Turbofish,
    #[at("/about")]
    About,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Turbofish => html! {<Turbofish/>},
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
