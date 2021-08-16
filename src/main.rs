use yew::{Component, ComponentLink, ShouldRender, Html, html, App};

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

impl Output {
    fn change_Str(&mut self, str: String) {
        self.props.str = str;
    }
}

impl Component for Output {
    type Message = Msg;
    type Properties = Properties;
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
                self.change_Str(str);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        }
        else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
            </div>
        }
    }
}



fn main() {
    App::<HelloWorld>::new().mount_to_body();
}
