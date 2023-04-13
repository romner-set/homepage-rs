use rand::Rng;
use crate::*;

const HIGHLIGHTED_STRINGS: &[&str] = &["&mut"];
const HIGHLIGHTED_CHARS: &str = "&<>()[];,";
const HIGHLIGHT_EXCEPTIONS: &[&str] = &["()"]; // Don't highlight unit type

enum MatchType {
    Exact, StartsWith, LastStartsWith, None
}

#[derive(Properties, PartialEq)]
pub struct TurbofishProps {
    #[prop_or_default]
    pub guts: String,
}

#[function_component()]
pub fn Turbofish(props: &TurbofishProps) -> Html {
    let mut rng = rand::prelude::thread_rng();
    html! {
        <>
            <title>{format!("::<{}>", props.guts)}</title>
            <main class="pond" aria-description={"turbofishes swimming across the screen"}>
                {{
                    {"turbofish"}.chars().map(|c| html! {
                        <@{c.to_string()} class={
                            if 0.5 > rng.gen

                                ::<f64> // A young Turbofish in its natural habitat.

                            () {"reverse"} else {""}
                        }>
                            <turbofish>
                            if !props.guts.is_empty() {{{
                                let mut buf = String::new();

                                html! {for props.guts.chars().map(|c| {
                                    buf.push(c);

                                    let mut match_type = MatchType::None;
                                    HIGHLIGHTED_STRINGS.iter().for_each(|&s|
                                        if s==&buf {match_type = MatchType::Exact; return}
                                        else if s.starts_with(&buf) {match_type = MatchType::StartsWith}
                                    );

                                    match match_type {
                                        MatchType::None => {
                                            HIGHLIGHT_EXCEPTIONS.iter().for_each(|&s|
                                                if s==&buf {match_type = MatchType::Exact; return}
                                                else if s.starts_with(&buf) {match_type = MatchType::StartsWith}
                                                else if s.starts_with(buf.chars().last().unwrap_or_default()) {match_type = MatchType::LastStartsWith}
                                            );
                                            match match_type {
                                                MatchType::Exact => html!{for buf.drain(..)},
                                                MatchType::StartsWith => html!{},
                                                MatchType::LastStartsWith => html! { // Special case for &(), &m() and &mu()
                                                    for buf.drain(..buf.len()-1).map(|_c| {
                                                        if HIGHLIGHTED_CHARS.contains(_c) {
                                                            html!{<span>{_c}</span>}
                                                        } else {
                                                            html!{_c}
                                                        }
                                                    })
                                                },
                                                MatchType::None => html! {
                                                    for buf.drain(..).map(|_c| {
                                                        if HIGHLIGHTED_CHARS.contains(_c) {
                                                            html!{<span>{_c}</span>}
                                                        } else {
                                                            html!{_c}
                                                        }
                                                    })
                                                }
                                            }
                                        },
                                        MatchType::Exact => html! {
                                            html!{<span>{for buf.drain(..)}</span>}
                                        },
                                        _ => html!{}
                                    }
                                })}
                            }}}
                            </turbofish>
                        </@>
                    }).collect
                        ::<Html> /* Once again, the awe-inspiring Turbofish! */ ()
                }}
            </main>
            <Footer refresh_button=true about_link=true root_link={!props.guts.is_empty()}/>
        </>
    }
}
