use crate::*;

#[derive(Properties, PartialEq)]
pub struct StatusCodeProps {
    #[prop_or(404)]
    pub code: u16,
}

#[function_component()]
pub fn StatusCode(props: &StatusCodeProps) -> Html {
    let navigator = use_navigator().unwrap();

    let code = if let Ok(c) = HttpStatusCode::from_u16(props.code) {c} else {
        HttpStatusCode::INTERNAL_SERVER_ERROR
    };

    let onclick = Callback::from(move |_| navigator.push(&Route::Root));
    html! {
        <>
            <main>
                <div class="status-code-head">
                    <turbofish>{code.as_str()}</turbofish>
                </div>
                <div class="status-code-body">{code.canonical_reason()}</div>
            </main>
            <footer>
                <button {onclick}>{"::<>"}</button>
                <a href="https://github.com/romner-set/turbo.fish">{"code."}</a>
            </footer>
        </>
    }
}
