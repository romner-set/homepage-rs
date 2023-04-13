use rand::Rng;
use crate::*;

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
                                    let mut mutref_buffer = String::new();
                                    let clear_mutrefb = |mb: &mut String, offset: usize| -> Html {
                                        html! {
                                            <>
                                                {{
                                                    mb.drain(..mb.len()-offset).map(|_c| if _c=='&' {
                                                        html! {<span>{'&'}</span>}
                                                    } else {
                                                        html! {_c}
                                                    }).collect::<Html>()
                                                }}
                                            </>
                                        }
                                    };
                                    props.guts.chars().map(|c| {
                                        match c { // I realize how ugly this code is, but it is currently 
                                                  // almost 2 AM and I am too sleep-deprived to care
                                            '&' => {
                                                mutref_buffer.push(c);
                                                if mutref_buffer.len()==2 {html!{}}
                                                else {clear_mutrefb(&mut mutref_buffer,1)}
                                            }
                                            'm' => {
                                                mutref_buffer.push(c);
                                                if mutref_buffer.len()==2 {html!{}}
                                                else {clear_mutrefb(&mut mutref_buffer,0)}
                                            }
                                            'u' => {
                                                mutref_buffer.push(c);
                                                if mutref_buffer.len()==3 {html!{}}
                                                else {clear_mutrefb(&mut mutref_buffer,0)}
                                            }
                                            't' => {
                                                mutref_buffer.push(c);
                                                if mutref_buffer.len()==4 {
                                                    html! {<span>{mutref_buffer.drain(..).collect::<String>()}</span>}
                                                } else {clear_mutrefb(&mut mutref_buffer,0)}
                                            }
                                            c => html! {
                                                <>
                                                    if !mutref_buffer.is_empty() {{clear_mutrefb(&mut mutref_buffer,0)}}
                                                    {match c {
                                                        '<'|'>'|','|'['|']'|';' => html! {<span>{c}</span>},
                                                        c => html! {c},
                                                    }}
                                                </>
                                            }
                                        }
                                    }).collect::<Html>()
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
