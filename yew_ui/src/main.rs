mod components;
mod pages;
mod router;

use crate::components::number_input::NumberInput;
use crate::components::text_input::TextInput;
use crate::router::{switch_main, MainRoute};
use gloo_net::http::Request;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use yew::prelude::*;
use yew_router::prelude::*;
// use yew::events::ClickEvent;
// use web_sys::HtmlInputElement;
// use std::clone;
use std::ops::Deref;
// use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Conductora {
    id: i32,
    nombre: String,
    edad: i32,
    telefono: String,
    correo: String,
    area: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConductoraSinId {
    nombre: String,
    edad: i32,
    telefono: String,
    correo: String,
    area: String,
}

#[derive(Default, Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<ConductoraSinId>,
}

#[function_component(App)]
fn app(props: &Props) -> Html {
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

    //crear variable para alojar la conductora que se va a agregar

    let conductora_handle = use_state(|| ConductoraSinId {
        nombre: "test".to_string(),
        edad: 0,
        telefono: "test".to_string(),
        correo: "test".to_string(),
        area: "test".to_string(),
    });

    // let conductora_clonada = conductora_handle.clone();
    // let nombre_cambiado = Callback::from(move |nombre| {
    //     let mut data = conductora_clonada.deref().clone();
    //     data.nombre = nombre;
    //     conductora_clonada.set(data);
    // });

    // let conductora_clonada = conductora_handle.clone();
    // let edad_cambiada = Callback::from(move |edad| {
    //     let mut data = conductora_clonada.deref().clone();
    //     data.edad = edad;
    //     conductora_clonada.set(data);
    // });

    // let conductora_clonada = conductora_handle.clone();
    // let telefono_cambiado = Callback::from(move |telefono| {
    //     let mut data = conductora_clonada.deref().clone();
    //     data.telefono = telefono;
    //     conductora_clonada.set(data);
    // });

    // let conductora_clonada = conductora_handle.clone();
    // let correo_cambiado = Callback::from(move |correo| {
    //     let mut data = conductora_clonada.deref().clone();
    //     data.correo = correo;
    //     conductora_clonada.set(data);
    // });

    // let conductora_clonada = conductora_handle.clone();
    // let area_cambiada = Callback::from(move |area| {
    //     let mut data = conductora_clonada.deref().clone();
    //     data.area = area;
    //     conductora_clonada.set(data);
    // });

    // let form_onsubmit = props.onsubmit.clone();
    // let conductora_clonada = conductora_handle.clone();
    // let onsubmit = Callback::from(move |_event: SubmitEvent| {
    //     // event.prevent_default();
    //     let data = conductora_clonada.deref().clone();
    //     form_onsubmit.emit(data);
    // });

    //now add a function to send the data to the database after the submit button is clicked
    let conductora_clonada = conductora_handle.clone();
    let form_submitted = {
        let data2 = conductora_clonada.deref().clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // let client = reqwest::Client::new();
            // let _ = client
            //     .post("http://localhost:8081/crearconductora")
            //     //insert test data
            //     .json(&ConductoraSinId {
            //         nombre: "test3".to_string(),
            //         edad: 25,
            //         telefono: "test3".to_string(),
            //         correo: "test3".to_string(),
            //         area: "test3".to_string(),
            //     })
            //     .send();
        })
    };

    // let form_submitted = {
    //         // let conductora = (*conductora_handle).clone();

    //         Callback::from(move |event: SubmitEvent| {
    //             event.prevent_default();

    //         wasm_bindgen_futures::spawn_local(async move {
    //             let client = reqwest::Client::new();
    //             let _ = client
    //                 .post("http://localhost:8081/crearconductora")
    //                 //insert data from the form inputs
    //                 .json(&ConductoraSinId {
    //                     nombre: (*conductora_handle).nombre,
    //                     edad: (*conductora_handle).edad,
    //                     telefono: (*conductora_handle).telefono,
    //                     correo: (*conductora_handle).correo,
    //                     area: (*conductora_handle).area

    //                 })
    //                 .send()
    //                 .await;
    //         });
    // })};

    html! {
        <>
        <h1>{ "Línea Rosa" }</h1>
        <div>
            <h3>{ "Conductoras" }</h3>
        </div>

        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>

        <div>

            // <form onsubmit={form_submitted}>

            //     <label>{ "Nombre: " }</label>
            //     <input id="nombre" name="nombre" type="text"  />
            //     <label>{ "Edad: " }</label>
            //     <input id="edad" name="edad" type="number"   />
            //     <label>{ "Teléfono: " }</label>
            //     <input id="telefono" name="telefono" type="text"  />
            //     <label>{ "Correo: " }</label>
            //     <input id="correo" name="correo" type="text" />
            //     <label>{ "Área: " }</label>
            //     <input id="area" name="area" type="text"   />

            //     // <label>{ "Nombre: " }</label>
            //     // <TextInput name="nombre" handle_onchange={nombre_cambiado} />

            //     // <label>{ "Edad: " }</label>
            //     // <NumberInput name="edad" handle_onchange_number={edad_cambiada} />

            //     // <label>{ "Teléfono: " }</label>
            //     // <TextInput name="telefono" handle_onchange={telefono_cambiado} />

            //     // <label>{ "Correo: " }</label>
            //     // <TextInput name="correo" handle_onchange={correo_cambiado} />

            //     // <label>{ "Área: " }</label>
            //     // <TextInput name="area" handle_onchange={area_cambiada} />

            //     <button type="submit" >{ "Agregar" }</button>
            // </form>

            // <form onsubmit={Callback::from(move |event: SubmitEvent| {
            //     event.prevent_default();
            //     crear_conductora(*conductora_handle.clone());
            // })}>
            //     <label>{ "Nombre: " }</label>
            //     <input
            //     type="text"

            //     oninput={Callback::from(move |event: InputEvent| {
            //         let conductora = conductora_handle.clone();
            //         conductora_handle.set(ConductoraSinId {
            //             //unwrap event data with option expect
            //             nombre: event.target_unchecked_into::<HtmlInputElement>().value().to_string(),
            //             edad: conductora.edad,
            //             telefono: conductora.telefono,
            //             correo: conductora.correo,
            //             area: conductora.area
            //         });
            //     })}
            // />

            // </form>

        </div>

        <div>
            <table>
                <thead>
                    <tr>
                        <th>{ "Nombre" }</th>
                        <th>{ "Edad" }</th>
                        <th>{ "Teléfono" }</th>
                        <th>{ "Correo" }</th>
                        <th>{ "Área" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for conductoras.iter().map(|conductora| html! {
                        <tr>
                            <td>{ &conductora.nombre }</td>
                            <td>{ &conductora.edad }</td>
                            <td>{ &conductora.telefono }</td>
                            <td>{ &conductora.correo }</td>
                            <td>{ &conductora.area }</td>
                        </tr>
                    }) }
                </tbody>
            </table>

        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// async fn crear_conductora(conductora: ConductoraSinId) {
//     wasm_bindgen_futures::spawn_local(async move {
//         Request::post("http://localhost:8081/crearconductora")
//             .header("Content-Type", "application/json")
//             .body(serde_json::to_string(&conductora).unwrap())
//             .unwrap()
//             .send()
//             .await
//             .unwrap();
//     });
// }

// async fn crear_conductora_reqwest(conductora: ConductoraSinId) -> Result<(), reqwest::Error> {
//     let client = reqwest::Client::new();
//     let data = serde_json::to_string(&conductora).unwrap();
//     let res = client
//         .post("http://localhost:8081/crearconductora")
//         .header("Content-Type", "application/json")
//         .body(data)
//         .send()
//         .await?;
//     Ok(())
// }

// fn handle_submit(conductora: ConductoraSinId, event: yew::SubmitEvent) {
//     wasm_bindgen_futures::spawn_local(async move {
//         Request::post("http://localhost:8081/crearconductora")
//             .header("Content-Type", "application/json")
//             .body(serde_json::to_string(&conductora).unwrap())
//             .unwrap()
//             .send()
//             .await
//             .unwrap();
//     });
// }
