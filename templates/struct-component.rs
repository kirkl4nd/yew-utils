use yew::{html, Component, Context, Html};

pub enum Msg {
    // component messages
}

pub struct StructComponent {
    // component state variables
}

impl Component for StructComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0, }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // match each message, return true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Struct Component Template"}</h1>
            </div>
        }
    }
}