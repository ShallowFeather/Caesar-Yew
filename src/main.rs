use yew::{Component, ComponentLink, ShouldRender, Html, html, App};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use input::TextInput;

mod input;
mod caesar;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct Properties {
    str: String,
}

pub enum Msg {
    Decrypt(String, u8),
    Encrypt(String, u8),
}

pub struct Output {
    link: ComponentLink<Self>,
    props: Properties,
}

impl Component for Output {
    type Message = Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: Properties {
                str: "owo".to_string(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decrypt(S, num) => {
                let res = caesar::decrypt(S, num);
                self.change_Str(res);
                true
            }
            Msg::Encrypt(S, num) => {
                let res = caesar::encrypt(S, num);
                self.change_Str(res);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <TextInput value="New post" onsubmit={self.link.callback(Msg::CreatePost)} />
                <button class="button" onclick={self.link.callback(|_| Msg::Decrypt)}>
                        { "decrypt" }
                </button>
                <button class="button" onclick={self.link.callback(|_| Msg::Encrypt)}>
                        { "encrypt" }
                </button>
            </div>
        }
    }
}



fn main() {
}
