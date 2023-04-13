use crate::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub about_link: bool,

    #[prop_or_default]
    pub root_link: bool,

    #[prop_or_default]
    pub refresh_button: bool,
}

#[function_component()]
pub fn Footer(props: &FooterProps) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Generic {
        path: format!("::<{}>", random::random_type(&mut rand::prelude::thread_rng(), 0))
    }));

    html! {
        <>
            <footer class="left">
                <button {onclick}>{"random!"}</button>
                if props.refresh_button {<a href="">{"⟳"}</a>}
            </footer>
            <footer class="right">
                if props.root_link {
                    <Link<Route> to={Route::Root}>{"::<>"}</Link<Route>>
                }
                if props.about_link {
                    <Link<Route> to={Route::About}>{"what?"}</Link<Route>>
                }
                <a href="https://github.com/romner-set/turbo.fish">{"code."}</a>
            </footer>
        </>
    }
}

