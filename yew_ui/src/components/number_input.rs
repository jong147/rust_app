use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange_number: Callback<i32>
}

#[function_component(NumberInput)]
pub fn number_input(props: &Props) -> Html {
    let handle_onchange_number = props.handle_onchange_number.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let value = value.parse::<i32>().unwrap();
        handle_onchange_number.emit(value);
    });

    html! {
      <input type="number" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}

