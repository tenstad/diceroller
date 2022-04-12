extern crate rand;

use gloo::events::EventListener;
use rand::prelude::*;
use std::fmt;
use yew::prelude::*;

struct Die(u8);

struct Dice(Die, Die);

struct Model {
    keyboard_listener: Option<EventListener>,
    mouse_listener: Option<EventListener>,
    history: Vec<Dice>,
    prerolls: Vec<Dice>,
}

enum Msg {
    Roll,
}

impl Model {
    fn preroll_all_combinations(&mut self) {
        for _ in 0..2 {
            for i in 1..7 {
                for j in 1..7 {
                    self.prerolls.push(Dice(Die(i), Die(j)))
                }
            }
        }
        let mut rng = rand::thread_rng();
        self.prerolls.shuffle(&mut rng);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keyboard_listener: None,
            mouse_listener: None,
            history: vec![],
            prerolls: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Roll => {
                let roll = match self.prerolls.pop() {
                    Some(roll) => roll,
                    _ => {
                        self.preroll_all_combinations();
                        self.prerolls.pop().unwrap()
                    }
                };
                self.history.push(roll);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="
                position: absolute;
                top: 50%;
                transform: translateY(-50%);
                -moz-transform: translateY(-50%);
                -webkit-transform: translateY(-50%);
                ">
                <p style="
                    margin: 0;
                    width: 100vw;
                    text-align: center;
                    user-select: none;
                    -moz-user-select: -moz-none;
                    -khtml-user-select: none;
                    -webkit-user-select: none;
                    -ms-user-select: none;
                    ">
                    
                    {
                        match self.history.last() {
                            Some(dice) => html! {
                                <>
                                    <span style="font-size: 45vw; color: #932724;">{format!("{}", dice.0)}</span>
                                    <span style="font-size: 45vw; color: #CFB809;">{format!("{}", dice.1)}</span>
                                </>
                            },
                            _ => html! {
                                <>
                                    <span  style="font-size: 10vw;">{"press 🖱️ or ⌨️"}</span>
                                </>
                            },
                        }
                    }
                </p>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let target = gloo_utils::window();

        let link = ctx.link().clone();
        let keyboard_listener = EventListener::new(&target, "keydown", move |_: &Event| {
            
            link.send_message(Msg::Roll)
        });
        self.keyboard_listener.replace(keyboard_listener);

        let link = ctx.link().clone();
        let mouse_listener = EventListener::new(&target, "mousedown", move |_: &Event| {
            link.send_message(Msg::Roll)
        });
        self.mouse_listener.replace(mouse_listener);

    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Die(1) => "⚀",
                Die(2) => "⚁",
                Die(3) => "⚂",
                Die(4) => "⚃",
                Die(5) => "⚄",
                Die(6) => "⚅",
                _ => "",
            }
        )
    }
}

fn main() {
    yew::start_app::<Model>();
}
