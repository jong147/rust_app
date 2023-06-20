use crate::router::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Inicio)]
pub fn inicio() -> Html {
    html! {
      <>
        <h1>{"Inicio"}</h1>
        <div>
          <Link<MainRoute> to={MainRoute::Inicio}>{ "Inicio" }</Link<MainRoute>>
          <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
        </div>
      </>
    }
}
