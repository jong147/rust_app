mod components;
mod pages;
mod router;

use crate::router::{switch_main, MainRoute};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

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
            // <form onsubmit={form_submitted}>
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
        </div>

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
                                <button>{ "Actualizar" }</button>
                                {"\u{00a0}"}
                                <button>{ "Eliminar" }</button>
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
    yew::Renderer::<App>::new().render();
}