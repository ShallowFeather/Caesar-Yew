use yew::{classes, html, web_sys::{Event, HtmlInputElement, HtmlTextAreaElement}, Callback,
          Component, ComponentLink, Html, Properties, ShouldRender};

pub enum Msg {
    Input(String, u8),
    Decrypt,
    Encrypt,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {

}

pub struct Post {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="code">
                    <input
                        class={classes!("new-client", "firstname")}
                        placeholder="First name"
                        onchange={update_name(Msg::UpdateFirstName)}
                    />
                </div>

                <button
                    disabled={client.first_name.is_empty() || client.last_name.is_empty()}
                    onclick={link.callback(|_| Msg::Add)}
                >
                    { "Add New" }
                </button>
                <button onclick={link.callback(|_| Msg::Abort)}>
                    { "Go Back" }
                </button>
            </>
        }
    }
}