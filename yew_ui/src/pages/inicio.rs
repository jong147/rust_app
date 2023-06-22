use crate::router::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Inicio)]
pub fn inicio() -> Html {
    html! {
      <>
        <h3>{"Inicio"}</h3>
        <div>
          <Link<MainRoute> to={MainRoute::Inicio}>{ "Inicio" }</Link<MainRoute>>
          <br/>
          <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
          <br/>
          <Link<MainRoute> to={MainRoute::ActualizarConductora}>{ "Actualizar" }</Link<MainRoute>>
        </div>
      </>
    }
}
