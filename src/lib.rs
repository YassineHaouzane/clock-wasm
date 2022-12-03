mod clock_utils;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("p")?;
    val.set_class_name("clock");
    body.append_child(&val)?;
    let cb = Closure::new(move || {
        let (hours, minutes, secs) = clock_utils::get_current_time();
        let s: &str = &clock_utils::get_time(hours, minutes, secs);
        let cleaned_str: &str = &s.replace(" ", "&nbsp");
        val.set_inner_html(cleaned_str);
    });

    setInterval(&cb, 1000);
    cb.forget();
    Ok(())
}
