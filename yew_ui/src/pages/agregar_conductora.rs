use crate::router::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Conductora {
    nombre: String,
    edad: i32,
    telefono: String,
    correo: String,
    area: String,
}

#[function_component(AgregarConductora)]
pub fn agregar_conductora() -> Html {

    let conductora_state = use_state(|| Conductora::default());

    //función para obtener el nombre de la conductora
    let conductora_clonada = conductora_state.clone();
    let nombre_handler = Callback::from(move |event: InputEvent| {
        let mut data = conductora_clonada.deref().clone();
        data.nombre = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        conductora_clonada.set(data);
    });

    //función para obtener la edad de la conductora
    let conductora_clonada = conductora_state.clone();
    let edad_handler = Callback::from(move |event: InputEvent| {
        let mut data = conductora_clonada.deref().clone();
        data.edad = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<i32>()
            .unwrap();
        conductora_clonada.set(data);
    });

    //función para obtener el teléfono de la conductora
    let conductora_clonada = conductora_state.clone();
    let telefono_handler = Callback::from(move |event: InputEvent| {
        let mut data = conductora_clonada.deref().clone();
        data.telefono = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        conductora_clonada.set(data);
    });

    //función para obtener el correo de la conductora
    let conductora_clonada = conductora_state.clone();
    let correo_handler = Callback::from(move |event: InputEvent| {
        let mut data = conductora_clonada.deref().clone();
        data.correo = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        conductora_clonada.set(data);
    });

    //función para obtener el área de la conductora
    let conductora_clonada = conductora_state.clone();
    let area_handler = Callback::from(move |event: InputEvent| {
        let mut data = conductora_clonada.deref().clone();
        data.area = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        conductora_clonada.set(data);
    });

    //función para agregar conductora
    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let conductora_clonada = conductora_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            Request::post("http://localhost:8081/crearconductora")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&*conductora_clonada).unwrap())
                .unwrap()
                .send()
                .await
                .unwrap();
        });
    });

    html! {
      <>
        <h3>{"Agregar Conductora"}</h3>
        <div>
          <Link<MainRoute> to={MainRoute::Inicio}>{ "Inicio" }</Link<MainRoute>>
          {"\u{2003}"}
          <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
        </div>
        <div>
            <br/>
            <form onsubmit={on_submit}>
                <label>{ "Nombre: " }</label>
                <input id="nombre" name="nombre" type="text" oninput={nombre_handler} />
                {"\u{00a0}"}
                <label>{ "Edad: " }</label>
                <input id="edad" name="edad" type="number" oninput={edad_handler} />
                {"\u{00a0}"}
                <label>{ "Teléfono: " }</label>
                <input id="telefono" name="telefono" type="text" oninput={telefono_handler} />
                {"\u{00a0}"}
                <label>{ "Correo: " }</label>
                <input id="correo" name="correo" type="text" oninput={correo_handler} />
                {"\u{00a0}"}
                <label>{ "Área: " }</label>
                <input id="area" name="area" type="text" oninput={area_handler} />
                <br/>
                <br/>
                <button type="submit">{ "Agregar" }</button>
            </form>
        </div>
      </>
    }
}
