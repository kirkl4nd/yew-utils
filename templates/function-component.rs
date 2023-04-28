use yew::prelude::*;

#[function_component]
fn FunctionComponent() -> Html {
    let state = use_state(|| /* state vars here */);

    let do_something = {
        let state = state.clone();
        Callback::from(move |_| state.set(/* new state */))
    };

    html! {
        <div>
            <h1>{"Function Component Template"}</h1>
        </div>
    }
}