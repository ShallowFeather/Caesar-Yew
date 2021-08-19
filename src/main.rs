use yew::prelude::*;
use yew::web_sys::HtmlTextAreaElement;

mod caesar;

enum Msg {
    Str(String),
    Num(u8),
    Decrypt,
    Encrypt,
}

struct Model {
    link: ComponentLink<Self>,
    output: String,
    str: String,
    num: u8,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            output: "owo".to_string(),
            str: "".to_string(),
            num: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decrypt => {
                self.output = caesar::decrypt(self.str.clone(), self.num.clone());
                true
            }
            Msg::Encrypt => {
                self.output = caesar::encrypt(self.str.clone(), self.num.clone());
                true
            }
            Msg::Str(str) => {
                self.str = str;
                true
            }
            Msg::Num(num) => {
                self.num = num;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    oninput=self.link.callback(|e: InputData|
                        Msg::Str(e.value)
                    )
                />
                <input
                    type="text"
                    oninput=self.link.callback(|e: InputData|
                        Msg::Num(e.value.parse().unwrap())
                    )
                />
                <button onclick=self.link.callback(|_| Msg::Decrypt)>{ "Decrypt" }</button>
                <button onclick=self.link.callback(|_| Msg::Encrypt)>{ "Encrypt" }</button>
                <p>{ &self.output } </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}