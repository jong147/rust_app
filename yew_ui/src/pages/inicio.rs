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
          {"\u{2003}"}
          <Link<MainRoute> to={MainRoute::AgregarConductora}>{ "Agregar" }</Link<MainRoute>>
        </div>
      </>
    }
}
