mod board;
mod game;
mod piece;

use yew::prelude::*;
use yew::{html, Component, Context, Html};
// Define the possible messages which can be sent to the component
pub enum Msg {
    Select,
}

pub struct App {
    selected: bool, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selected: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select => {
                self.selected = !self.selected;
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let btn = html! { <button class="button" onclick={ctx.link().callback(|_| Msg::Select)}>
            { "Not" }
        </button>};

        html! {
            <div>
                <table>
                    <tr>
                        <td>
                            { btn.clone() }
                        </td>
                        <td>
                            { btn.clone() }
                        </td>
                    </tr>
                    <tr>
                        <td>
                            { btn.clone() }
                        </td>
                        <td>
                            { btn.clone() }
                        </td>
                    </tr>
                </table>


                // Display the current value of the counter
                <p>
                    { self.selected }
                </p>
            </div>

        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
