use yew::prelude::*;
use yew_router::prelude::*;
use http::status::StatusCode as HttpStatusCode;

mod pages;
use pages::{about::About, turbofish::Turbofish, status_code::StatusCode};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,

 // #[at("/status/:path")]
 // Status {path: String},

    #[at("/about")]
    About,

    #[at("/*path")]
    Generic {path: String},
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! {<Turbofish/>},
     // Route::Status{path} =>
     //     if let Ok(code) = HttpStatusCode::from_u16(path.parse().unwrap_or_default()) {
     //         html! {<StatusCode code={code.as_u16()}/>}
     //     } else {
     //         html! {<StatusCode code=404/>}
     //     }
        Route::Generic{path} => 
            if path.is_empty() {
                html! {<Turbofish/>}
            } else {
                let dpath = urlencoding::decode(&path).unwrap_or_default().into_owned();

                if dpath.chars().last() == Some('>') // Has to be first, the second check
                && &dpath.as_str()[..3] == "::<" {   // errors if run on an empty path
                    html! {<Turbofish guts={dpath[3..dpath.len()-1].to_owned()}/>}
                } else if path.chars().next() == Some('<')
                && &dpath.as_str()[dpath.len()-3..] == ">::" {
                    html! {<Turbofish guts={dpath[1..dpath.len()-3].to_owned()}/>}
                } else {
                    html! {<StatusCode code=404/>}
                }
            }
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
