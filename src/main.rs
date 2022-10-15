use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    html!{
        <div>
            <p>{"Hello world"}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
