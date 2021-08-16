use yew::{Component, ComponentLink, ShouldRender, Html};

mod input;
mod caesar;

pub enum Msg {
    Decrypt(String, u8),
    Encrypt(String, u8),
    Delete,
}

pub struct Model {
    link: ComponentLink<Self>,
    
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: PostId,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decrypt(S, num) => {
                let res = caesar::decrypt(S, num);
                true
            }
            Msg::Encrypt(S, num) => {
                let res = caesar::encrypt(S, num);

                true
            }
            Msg::Delete => {

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}


fn main() {

}
