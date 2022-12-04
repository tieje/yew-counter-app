use yew::prelude::*;

struct Model {
    value: i32,
}

enum Action {
    AddThree,
    SubtractTwo,
}

fn increase(state: UseStateHandle<Model>) -> Box<dyn Fn(MouseEvent)> {
    Box::new(move |_| {
        state.set(Model {
            value: state.value + 1,
        })
    })
}
fn increase_by_two(state: UseStateHandle<Model>) -> Box<dyn Fn(MouseEvent)> {
    Box::new(move |_| {
        state.set(Model {
            value: state.value + 2,
        })
    })
}
// helper function for increase_or_decrease
fn add_by(state: UseStateHandle<Model>, amt: i32) -> Box<dyn Fn(MouseEvent)> {
    Box::new(move |_| {
        state.set(Model {
            value: state.value + amt,
        })
    })
}
// Addition and subtraction can be simplified to being calculated without specific actions.
// Actions are more of redux-like way of determining what to do with state.
// This redux-like abstraction can handle a lot more possible changes to state and will be left here as a reference.
fn increase_or_decrease(
    state: UseStateHandle<Model>,
    action: Action,
    amt: i32,
) -> Box<dyn Fn(MouseEvent)> {
    match action {
        Action::SubtractTwo => add_by(state, amt),
        Action::AddThree => add_by(state, amt),
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| Model { value: 0 });

    // Initial Normal way
    let decrease = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(Model {
                value: counter.value - 1,
            })
        })
    };
    // extract set state to function
    let increase = {
        let counter = counter.clone();
        Callback::from(increase(counter))
    };
    // extract clone
    let increase_by_two = increase_by_two(counter.clone());

    // Next is an external function that can increase or decrease state depending on Input.
    // This ended up being fn increase_or_decrease().

    html! {
        <div>
            <button onclick={increase}>{ "+1" }</button>
            <button onclick={increase_by_two}>{ "+2" }</button>
            <button onclick={increase_or_decrease(counter.clone(), Action::AddThree, 3)}>{"+3"}</button>
            <button onclick={decrease}>{ "-1" }</button>
            <button onclick={increase_or_decrease(counter.clone(), Action::SubtractTwo, -2)}>{"-2"}</button>
            <button onclick={increase_or_decrease(counter.clone(), Action::SubtractTwo, -3)}>{"-3"}</button>
            <p>{ counter.value }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
