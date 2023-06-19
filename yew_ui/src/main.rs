use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

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
    let conductoras_buscadas: Vec<Conductora> = Request::get("http://localhost:8081/conductoras")
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
        <h1>{ "Línea Rosa" }</h1>
        <div>
            <h3>{ "Conductoras" }</h3>
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