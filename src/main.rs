use yew::prelude::*;

struct Model {
    value: i32,
}



fn increase(state: UseStateHandle<Model>) -> Box<dyn Fn(MouseEvent)>{
    Box::new(move |_| {
        state.set(Model {
            value: state.value + 1,
        })
    })
}
fn increase_by_two(state: UseStateHandle<Model>) -> Box<dyn Fn(MouseEvent)>{
    Box::new(move |_| {
        state.set(Model {
            value: state.value + 2,
        })
    })
}

// fn increase_or_decrease(state: UseStateHandle<Model>) -> Box<dyn Fn(MouseEvent)>{
//     Box::new(move |_| {
//         state.set(Model {
//             value: state.value + 2,
//         })
//     })
// }

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
    // function that can increase or decrease state depending on Input
    
    html! {
        <div>
            <button onclick={increase}>{ "+1" }</button>
            <button onclick={increase_by_two}>{ "+2" }</button>
            <button onclick={decrease}>{ "-1" }</button>
            <p>{ counter.value }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
