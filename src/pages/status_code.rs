use crate::*;

#[derive(Properties, PartialEq)]
pub struct StatusCodeProps {
    #[prop_or(404)]
    pub code: u16,
}

#[function_component()]
pub fn StatusCode(props: &StatusCodeProps) -> Html {
    let code = if let Ok(c) = HttpStatusCode::from_u16(props.code) {c} else {
        HttpStatusCode::INTERNAL_SERVER_ERROR
    };

    html! {
        <>
            <title>{format!("::<{}> | {}", code.as_str(), code.canonical_reason().unwrap_or_default())}</title>
            <main>
                <div class="status-code-head">
                    <turbofish>{code.as_str()}</turbofish>
                </div>
                <div class="status-code-body">{code.canonical_reason()}</div>
            </main>
            <Footer root_link=true about_link=true refresh_button=true/>
        </>
    }
}
