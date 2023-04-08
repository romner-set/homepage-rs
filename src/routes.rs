use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use minijinja::Environment;
use serde::Serialize;

pub async fn index(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
    render_html_template(&env, "index", ())
}

pub async fn about(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
    render_html_template(&env, "about", ())
}

fn render_html_template<S>(
    env: &Environment<'_>,
    name: &str,
    ctx: S,
) -> Result<Html<String>, impl IntoResponse>
where
    S: Serialize,
{
    let Ok(template) = env.get_template(name) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't find MiniJinja template".to_owned(),
        ));
    };

    let rendered = template.render(ctx).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render MiniJinja template: {e}"))
    })?;

    Ok(Html(rendered))
}
