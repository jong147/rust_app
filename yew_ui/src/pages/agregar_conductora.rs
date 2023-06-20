use crate::router::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AgregarConductora)]
pub fn agregar_conductora() -> Html {
    html! {
      <>
        <h1>{"Agregar Conductora"}</h1>
        <div>
          <Link<MainRoute> to={MainRoute::Inicio}>{ "Inicio" }</Link<MainRoute>>
          <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
        </div>
        <div>
            <form>
                <label>{ "Nombre: " }</label>
                <input id="nombre" type="text" />
                <label>{ "Edad: " }</label>
                <input id="edad" type="number"/>
                <label>{ "Teléfono: " }</label>
                <input id="telefono" type="text" />
                <label>{ "Correo: " }</label>
                <input id="correo" type="text" />
                <label>{ "Área: " }</label>
                <input id="area" type="text" />
                <button type="submit">{ "Agregar" }</button>
            </form>
        </div>
      </>
    }
}