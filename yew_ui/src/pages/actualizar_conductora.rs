use crate::router::MainRoute;
use yew::prelude::*;
use yew_router::{prelude::*, history::{BrowserHistory, History}};

use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use std::ops::Deref;
use web_sys::{HtmlInputElement};
use wasm_bindgen::JsCast;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Conductora {
    id: i32,
    nombre: String,
    edad: i32,
    telefono: String,
    correo: String,
    area: String,
}

#[function_component(ActualizarConductora)]
pub fn actualizar_conductora() -> Html {
    
    let conductora_state = use_state(|| Conductora::default());
    
    let conductora_clonada = conductora_state.clone();

    let history = BrowserHistory::new();
    let history = history.location().clone();
    let route_id: String = history.path().split("/").last().unwrap().to_string();

    //función para obtener el nombre de la conductora
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

    //función para actualizar conductora
    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let conductora_clonada = conductora_state.clone();
        let conductora_id = route_id.clone();

        conductora_clonada.set(Conductora {
            id: conductora_id.clone().parse::<i32>().unwrap(),
            nombre: conductora_clonada.deref().nombre.clone(),
            edad: conductora_clonada.deref().edad.clone(),
            telefono: conductora_clonada.deref().telefono.clone(),
            correo: conductora_clonada.deref().correo.clone(),
            area: conductora_clonada.deref().area.clone(),
        });

        wasm_bindgen_futures::spawn_local(async move {
            Request::put((format!("{}{}", "http://localhost:8081/conductoras/", conductora_id.clone())).as_str())
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
        <h3>{"Actualizar Conductora"}</h3>
        <div>
            <Link<MainRoute> to={MainRoute::Inicio}>{ "Inicio" }</Link<MainRoute>>
            {"\u{2003}"}
            <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
        </div>
        <div>
            <br/>
            //no esta funcionando actualmente, en modo de prueba
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
                <button type="submit">{ "Actualizar" }</button>
            </form>
        </div>
      </>
    }
}
