mod board;
mod game;
mod piece;

use stylist::Style;
use yew::prelude::*;
use yew::{function_component, html, Component, Context, Html};
// Define the possible messages which can be sent to the component
pub enum Msg {
    Select((i8, i8)),
}

pub struct App {
    selected: Option<(i8, i8)>,
    game: game::Game<'static>,
}

pub struct State {
    selected: Option<(i8, i8)>,
    game: game::Game<'static>,
}

#[function_component]
fn FunctionalApp() -> Html {
    let state = use_state(|| State {
        selected: None,
        game: game::Game::new(),
    });

    let cb = |(x, y)| {
        let state = state.clone();
        let s = *state.clone();
        s.selected = match state.selected {
            Some((x, y)) => None,
            None => Some((x, y)),
        };
        Callback::from(move |_| state.set(*state))
    };

    html! {
        <div>
            <table>
            {
            for (0..4).map(|row| {
                html! {
                <tr>
                {
                for (0..4).map(|column| {
                        html! {
                        <td>
                        <button class="button" onclick={cb((row, column))}>
                        { "Not" }
                        </button>
                        </td>
                        }
                })
                }
                </tr>
                }
            })
            }
            </table>


            <p>
            {
                if let Some(pos) = state.selected {
                    format!("Selected: {:?}", pos)
                } else {
                    "No selection".to_string()
                }
            }
            </p>
        </div>
    }
}

#[function_component]
fn CounterApp() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
        </>
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected: None,
            game: game::Game::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(pos) => {
                if self.selected == Some(pos) {
                    self.selected = None;
                } else {
                    // TODO: should move here
                    self.selected = Some(pos);
                }
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <table>
                {
                for (0..4).map(|row| {
                    html! {
                    <tr>
                    {
                    for (0..4).map(|column| {
                            html! {
                            <td>
                            <button class="button" onclick={ctx.link().callback(move |_| Msg::Select((row, column)))}>
                            { "Not" }
                            </button>
                            </td>
                            }
                    })
                    }
                    </tr>
                    }
                })
                }
                </table>


                <p>
                {
                    if let Some(pos) = self.selected {
                        format!("Selected: {:?}", pos)
                    } else {
                        "No selection".to_string()
                    }
                }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
