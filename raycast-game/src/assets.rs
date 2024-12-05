use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys::Uint8Array, Blob, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn get_asset(name: String, window: &web_sys::Window) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let url = format!("http://localhost:8080/{}", name);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into()?;
    let blob_js_val = JsFuture::from(resp.blob()?).await?;

    return Ok(blob_js_val);
}
