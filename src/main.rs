mod board;
mod game;
mod piece;

use yew::prelude::*;
use yew::{html, Component, Context, Html};

use crate::board::Coordinate;
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
            (Msg::Coordinate(pos), Some(Msg::Piece(p))) => {
                match self.game.put(p, pos.into()) {
                    Ok(_) => {}
                    Err(e) => self.game_error = format!("{:?}", e),
                }
                self.selected = None;
            }
            (Msg::Piece(p), None) => {
                self.selected = Some(Msg::Piece(p));
            }
            (Msg::Coordinate(from), None) => {
                if self.game.get_top_piece(from.into()).is_some() {
                    self.selected = Some(Msg::Coordinate(from));
                } else {
                    self.game_error = "No piece selected".to_string();
                }
            }
            (Msg::Piece(p), Some(Msg::Coordinate(pos))) => {
                self.selected = Some(Msg::Piece(p)); // we flush the selected position
            }
            (Msg::Coordinate(to), Some(Msg::Coordinate(from))) => {
                if from == to {
                    self.selected = None;
                } else {
                    match self.game.move_top(from.into(), to.into()) {
                        Ok(_) => {}
                        Err(e) => self.game_error = format!("{:?}", e),
                    }
                    self.selected = None;
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

        let from_row = self
            .game
            .hive()
            .iter()
            .map(|Coordinate { x: _, y }| *y)
            .min()
            .unwrap_or(0);

        let to_row = self
            .game
            .hive()
            .iter()
            .map(|Coordinate { x: _, y }| *y)
            .max()
            .unwrap_or(0);

        let from_column = self
            .game
            .hive()
            .iter()
            .map(|Coordinate { x, y: _ }| *x)
            .min()
            .unwrap_or(0)
            - 2;

        let to_column = self
            .game
            .hive()
            .iter()
            .map(|Coordinate { x, y: _ }| *x)
            .max()
            .unwrap_or(0)
            + 2;

        let board = html! {
            <div>
            { for ((from_row/2)*2-2..to_row+2).step_by(2).map(|row| {
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

            <div class="container">
                <div class="row">

                <div class="col">
                {
                    for blacks.map(|piece|{
                            html!{
                                <button class="button" onclick={
                                    let piece = piece.clone(); // TODO: what is the right way to do this?
                                    ctx.link().callback(move |_| Msg::Piece(piece.clone()))
                                }>
                                { format!("{}", piece) }
                                </button>
                            }
                        })
                }
                </div>


                <div class="col">
                {
                    for whites.map(|piece|{
                            html!{
                                <button class="button" onclick={
                                    let piece = piece.clone(); // TODO: what is the right way to do this?
                                    ctx.link().callback(move |_| Msg::Piece(piece.clone()))
                                }>
                                { format!("{}", piece) }
                                </button>
                            }
                        })
                }
                </div>


                </div>
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
