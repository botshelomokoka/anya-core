// src/ui/web_interface.rs
use yew::prelude::{Component, ComponentLink, Html, ShouldRender, html};

pub struct WebInterface {
    stacks_rpc: StacksRpc,
    user_session: Session,
}

impl Component for WebInterface {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            stacks_rpc: StacksRpc::new(), // Initialize the StacksRpc instance
            user_session: Session::new(), // Initialize a new user session
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Anya Core Web Interface"}</h1>
                // Add more UI components here
            </div>
        }
    }
}
