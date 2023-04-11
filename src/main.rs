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
        // TODO: there's a weird thing happening when from_row is odd
        let from_row = -6;
        let to_row = 4;
        let from_column = -5;
        let to_column = 5;

        let board = html! {
            <div>
            { for (from_row..to_row).step_by(2).map(|row| {
                html! {
                <div class="move">
                <div class="sangria move-vertically">
                { for ((from_column - row / 2 + 1)..to_column).map(|column| {
                        html! {
                        <button class="tile hex" onclick={ctx.link().callback(move |_| Msg::Coordinate((column, row)))}>
                        {
                            self.game.get_top_piece((column, row).into()).map(|p| format!("{p}\n({column},{row})")).unwrap_or(format!("({column},{row})"))
                        }
                        </button>
                        }
                })}
                </div>

                <div>
                { for ((from_column - row / 2)..to_column).map(|column| {
                        let row = row + 1;

                        html! {
                        <button class="tile hex" onclick={ctx.link().callback(move |_| Msg::Coordinate((column, row)))}>
                        {
                            self.game.get_top_piece((column, row).into()).map(|p| format!("{p}\n({column},{row})")).unwrap_or(format!("({column},{row})"))
                        }
                        </button>
                        }
                })}
                </div>
                </div>
            }})}
            </div>
        };

        let pool = self.game.get_pool();
        let whites = pool.iter().filter(|p| p.color == piece::Color::White);
        let blacks = pool.iter().filter(|p| p.color == piece::Color::Black);

        html! {
            <div>

                {board}
                <p>
                {
                    if let Some(pos) = &self.selected {
                        format!("Selected: {:?}", pos)
                    } else {
                        "No selection".to_string()
                    }
                }
                </p>

            <table>
            {
                for blacks.zip(whites).map(
                    |(black, white)| html!{
                        <tr>
                        {
                            for [black, white].map(|piece|{
                                html!{
                                    <td>
                                    <button class="button" onclick={
                                        let piece = piece.clone(); // TODO: what is the right way to do this?
                                        ctx.link().callback(move |_| Msg::Piece(piece.clone()))
                                    }>
                                    { format!("{}", piece) }
                                    </button>
                                    </td>
                                }
                            })
                        }
                        </tr>
                    }
                )
            }
            </table>

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
