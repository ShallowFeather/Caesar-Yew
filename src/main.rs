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
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
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
            output: "".to_string(),
            str: "".to_string(),
            num: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decrypt => {
                self.output = caesar::decrypt(self.str.clone(), self.num);
                true
            }
            Msg::Encrypt => {
                self.output = caesar::encrypt(self.str.clone(), self.num);
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
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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
                    type="number" min="0" max="26"
                    oninput=self.link.callback(|e: InputData|
                        Msg::Num(e.value.into_bytes())
                    )
                />
                <button onclick=self.link.callback(|_| Msg::Decrypt)>{ "Decrypt" }</button>
                <button onclick=self.link.callback(|_| Msg::Encrypt)>{ "Encrypt" }</button>
                <p>{ self.output } </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}