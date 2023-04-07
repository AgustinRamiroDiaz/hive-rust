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
    game: game::Game<'static>,
    game_error: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // TODO: ownership is hard here because the component requires 'static
        let pool = vec![
            &piece::Piece {
                bug: piece::Bug::Bee,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Beetle,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Beetle,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Spider,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Spider,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::Black,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::Black,
            },
            // Now the white pieces
            &piece::Piece {
                bug: piece::Bug::Bee,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Beetle,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Beetle,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Spider,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Spider,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Ant,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::White,
            },
            &piece::Piece {
                bug: piece::Bug::Grasshopper,
                color: piece::Color::White,
            },
        ];

        Self {
            selected: None,
            game: game::Game::new(pool),
            game_error: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.game_error = "".to_string();
        match (msg, self.selected.clone()) {
            (Msg::Coordinate(pos), Some(Msg::Piece(p))) => {
                todo!();

                match self.game.put(&p, pos.into()) {
                    Ok(_) => {
                        self.selected = None;
                    }
                    Err(e) => self.game_error = format!("{:?}", e),
                }
            }
            (s, None) => {
                self.selected = Some(s);
            }
            (Msg::Piece(p), Some(Msg::Coordinate(pos))) => {
                self.selected = Some(Msg::Piece(p)); // we flush the selected position
            }
            (Msg::Coordinate(to), Some(Msg::Coordinate(from))) => {
                todo!();

                match self.game.move_top(from.into(), to.into()) {
                    Ok(_) => {
                        self.selected = None;
                    }
                    Err(e) => self.game_error = format!("{:?}", e),
                }
            }
            (Msg::Piece(p), Some(Msg::Piece(_))) => {
                self.selected = Some(Msg::Piece(p)); // we flush the selected piece
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
                                let piece = piece.clone().to_owned(); // TODO: what is the right way to do this?
                                ctx.link().callback(move |_| Msg::Piece(piece.clone()))
                            }>
                            { format!("{:#?}", piece) }
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
