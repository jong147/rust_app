use gloo_net::http::Request;
use web_sys::MouseEvent;
use yew::Callback;

pub fn eliminar_fn(id: i32) -> Callback<MouseEvent> {
    let callback = Callback::from(move |event: MouseEvent| {
        event.prevent_default();

        wasm_bindgen_futures::spawn_local(async move {
            Request::delete((format!("{}{}", "http://localhost:8081/conductoras/", id)).as_str())
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&id).unwrap())
                .unwrap()
                .send()
                .await
                .unwrap();
        });
    });
    callback
}