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
        let mut cantidad_de_sangria = 0;

        let board = html! {
            <div>
            { for (-5..5).step_by(2).map(|row| {
                html! {
                <div class="move">
                <div class="sangria move-vertically">
                { for (-5..5).map(|column| {
                        html! {
                        <button class="tile hex" onclick={ctx.link().callback(move |_| Msg::Coordinate((row, column)))}>
                        {
                            self.game.get_top_piece((row, column).into()).map(|p| format!("{p}\n({row},{column})")).unwrap_or(format!("({row},{column})"))
                        }
                        </button>
                        }
                })}
                </div>

                <div>
                { for (-5..5).map(|column| {
                        html! {
                        <button class="tile hex" onclick={ctx.link().callback(move |_| Msg::Coordinate((row + 1, column)))}>
                        {
                            self.game.get_top_piece((row + 1, column).into()).map(|p| format!("{p}\n({},{column})", row +1)).unwrap_or(format!("({},{column})", row + 1))
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
