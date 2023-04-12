use rand::Rng;
use crate::*;

#[derive(Properties, PartialEq)]
pub struct TurbofishProps {
    #[prop_or_default]
    pub guts: String,
}

#[function_component()]
pub fn Turbofish(props: &TurbofishProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::About));
    html! {
        <>
            <main class="pond" aria-description={"turbofishes swimming across the screen"}>
                {{
                    let mut rng = rand::prelude::thread_rng();
                    {"turbofish"}.chars().map(|c| html! {
                        <@{c.to_string()} class={
                            if 0.5 > rng.gen

                                ::<f64> // A young Turbofish in its natural habitat.

                            () {"reverse"} else {""}
                        }><turbofish>{&props.guts}</turbofish></@>
                    }).collect
                        ::<Html> /* Once again, the awe-inspiring Turbofish! */ ()
                }}
            </main>
            <footer>
                <button {onclick}>{"what?"}</button>
                <a href="https://github.com/romner-set/turbo.fish">{"code."}</a>
            </footer>
        </>
    }
}
