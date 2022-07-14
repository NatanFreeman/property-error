use gloo_utils::{document, window};
use yew::prelude::*;

#[function_component(Main)]
pub fn main(_props: &()) -> Html {
    let root = document().query_selector(":root").unwrap().unwrap();

    let style = window().get_computed_style(&root).unwrap().unwrap();

    let progress = style.get_property_value("--color").unwrap();
    println!("{progress}");

    style.set_property("--color", "blue").unwrap();

    return html! {
        <>
            <h1 id="style">{"Hello, World!"}</h1>
        </>
    };
}

fn main() {
    let _ = yew::start_app::<Main>();
}
