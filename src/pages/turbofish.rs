use crate::*;

#[derive(Properties, PartialEq)]
pub struct TurbofishProps {
    #[prop_or_default]
    pub path: String,
}

#[function_component()]
pub fn Turbofish(props: &TurbofishProps) -> Html {
    let navigator = use_navigator().unwrap();

    let mut guts = String::new();

    if props.path.chars().last() == Some('>') // Has to be first, the second check
    && &props.path.as_str()[..3] == "::<" {   // errors if run on an empty path
        guts.push_str(&props.path[3..props.path.len()-1]);
    }

    let onclick = Callback::from(move |_| navigator.push(&Route::About));
    html! {
        <>
            <main class="pond" aria-description={"turbofishes swimming across the screen"}>
                {
                    {"turbofish"}.chars().map(|c| html! {
                        <@{c.to_string()}>{format!("<{guts}>")}</@>
                    }).collect
                    ::<Html> /* A Turbofish in its natural habitat. */ ()
                }
            </main>
            <footer>
                <button {onclick}>{"what?"}</button>
                <a href="https://github.com/romner-set/turbo.fish">{"code."}</a>
            </footer>
        </>
    }
}
