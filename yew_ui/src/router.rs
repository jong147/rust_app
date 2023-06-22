use crate::pages::agregar_conductora::AgregarConductora;
use crate::pages::actualizar_conductora::ActualizarConductora;
use crate::pages::inicio::Inicio;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Inicio,
    #[at("/crear_conductora")]
    AgregarConductora,
    // #[at("/actualizar_conductora/:id")]
    #[at("/actualizar_conductora")]
    ActualizarConductora,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Inicio => html! {< Inicio />},
        MainRoute::AgregarConductora => html! {< AgregarConductora />},
        MainRoute::ActualizarConductora => html! {< ActualizarConductora />},
        MainRoute::NotFound => html! {<h1>{"404"}</h1>},
    }
}
