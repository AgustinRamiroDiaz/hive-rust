mod board;
mod game;
mod piece;

use yew::prelude::*;
use yew::{html, Component, Context, Html};
// Define the possible messages which can be sent to the component
#[derive(Debug, Clone)]
enum Msg {
    Coordinate((i8, i8)),
    Piece(piece::Piece), // TODO: this should be a reference
}

struct App {
    selected: Option<Msg>,
    game: game::Game,
    game_error: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected: None,
            game: game::Game::new(game::Game::default_pool()),
            game_error: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.game_error = "".to_string();
        match (msg, self.selected.clone()) {
            (Msg::Coordinate(pos), Some(Msg::Piece(p))) => match self.game.put(p, pos.into()) {
                Ok(_) => {
                    self.selected = None;
                }
                Err(e) => self.game_error = format!("{:?}", e),
            },
            (s, None) => {
                self.selected = Some(s);
            }
            (Msg::Piece(p), Some(Msg::Coordinate(pos))) => {
                self.selected = Some(Msg::Piece(p)); // we flush the selected position
            }
            (Msg::Coordinate(to), Some(Msg::Coordinate(from))) => {
                if from == to {
                    self.selected = None;
                } else {
                    match self.game.move_top(from.into(), to.into()) {
                        Ok(_) => {
                            self.selected = None;
                        }
                        Err(e) => self.game_error = format!("{:?}", e),
                    }
                }
            }
            (Msg::Piece(p), Some(Msg::Piece(old))) => {
                if old == p {
                    self.selected = None;
                } else {
                    self.selected = Some(Msg::Piece(p)); // we flush the selected piece
                }
            }
        }
        true // Return true to cause the displayed change to update
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
                            <button class="button" onclick={ctx.link().callback(move |_| Msg::Coordinate((row, column)))}>
                            {
                                self.game.get_top_piece((row, column).into()).map(|p| format!("{}", p)).unwrap_or("".to_string())
                            }
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
                    if let Some(pos) = &self.selected {
                        format!("Selected: {:?}", pos)
                    } else {
                        "No selection".to_string()
                    }
                }
                </p>

                <div>
                {
                    for self.game.get_pool().iter().map(|piece|
                        html! {

                            <button class="button" onclick={
                                let piece = piece.clone(); // TODO: what is the right way to do this?
                                ctx.link().callback(move |_| Msg::Piece(piece.clone()))
                            }>
                            { format!("{}", piece) }
                            </button>
                        }
                    )
                }
                </div>

                <p>
                {
                    self.game_error.to_string()
                }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
