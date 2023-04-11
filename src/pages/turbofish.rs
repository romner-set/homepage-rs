use crate::*;

#[function_component()]
pub fn Turbofish() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::About));
    html! {
        <>
            <main class="pond" aria-description={"turbofishes swimming across the screen"}>
                {
                    {"turbofish"}.chars().map(|c| html! {
                        <@{c.to_string()}>{"<>"}</@>
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
