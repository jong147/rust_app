mod components;
mod pages;
mod router;

use crate::router::{switch_main, MainRoute};
use crate::pages::eliminar_conductora;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{prelude::*, BrowserRouter};

// use log::info;
// use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Conductora {
    id: i32,
    nombre: String,
    edad: i32,
    telefono: String,
    correo: String,
    area: String,
}

#[function_component(App)]
fn app() -> Html {

    //crear variable para alojar las conductoras de la base de datos
    let conductoras = use_state(|| vec![]);

    //buscar las conductoras en la base de datos
    {
        let conductoras = conductoras.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let conductoras_buscadas: Vec<Conductora> =
                Request::get("http://localhost:8081/conductoras")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            conductoras.set(conductoras_buscadas);
        });
    }

    html! {
        <>
        <center>
            <h1>{ "Línea Rosa" }</h1>

        <div>
            <h2>{ "Conductoras" }</h2>
        </div>

        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>

        <div>
            <br/>
            <table>
                <thead>
                    <tr>
                        <th>{ "Id" }</th>
                        <th>{ "Nombre" }</th>
                        <th>{ "Edad" }</th>
                        <th>{ "Teléfono" }</th>
                        <th>{ "Correo" }</th>
                        <th>{ "Área" }</th>
                        <th>{ "Opciones" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for conductoras.iter().map(|conductora| html! {
                        <tr>
                            <td>{ &conductora.id }</td>
                            <td>{ &conductora.nombre }</td>
                            <td>{ &conductora.edad }</td>
                            <td>{ &conductora.telefono }</td>
                            <td>{ &conductora.correo }</td>
                            <td>{ &conductora.area }</td>
                            <td>
                                <BrowserRouter>
                                    <button><Link<MainRoute> to={MainRoute::ActualizarConductora{id:conductora.id.to_string()}}>{ "Actualizar" }</Link<MainRoute>></button>
                                    {"\u{00a0}"}
                                    <button onclick={eliminar_conductora::eliminar_fn(conductora.id)} >{ "Eliminar" }</button>
                                </BrowserRouter>
                            </td>
                        </tr>
                    }) }
                </tbody>
            </table>

        </div>
        </center>
    </>
    }
}

fn main() {

    // Debugging con la consola del navegador
    // wasm_logger::init(wasm_logger::Config::default());
    // let object = JsValue::from("");
    // info!("Console: {}", object.as_string().unwrap());

    yew::Renderer::<App>::new().render();
}