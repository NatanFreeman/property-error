use core::result::Result;
use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleRule, CssStyleSheet};
use yew::prelude::*;

#[function_component(Main)]
pub fn main(_props: &()) -> Html {
    let style_sheet = document().style_sheets().item(0).unwrap();
    let css_style_object: Result<CssStyleSheet, _> = style_sheet.dyn_into();
    let rule: CssStyleRule = css_style_object
        .unwrap()
        .css_rules()
        .unwrap()
        .item(0)
        .unwrap()
        .dyn_into()
        .unwrap();
    let style = rule.style();
    style.set_property("--color", "black").unwrap();

    return html! {
        <>
            <h1 id="style">{"Hello, World!"}</h1>
        </>
    };
}

fn main() {
    let _ = yew::start_app::<Main>();
}
