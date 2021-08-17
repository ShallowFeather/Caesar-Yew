use yew::prelude::*;

mod caesar;

enum Msg {
    Decrypt,
    Encrypt,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    str: String,
    num: u8,
    output: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            str: "".to_string(),
            num: 0,
            output: "".to_string(),
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
                    oninput={self.link.callback(|e: InputEvent| {
                    let input: HtmlTextAreaElement = e.target_unchecked_into();
                    self.str={self.}
            })}
                    {self.str}
                />
                <input
                    type="number" min="0" max="26"
                    {self.num}/>
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