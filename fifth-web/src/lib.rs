//! WASM bindings for Fifth

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;

use miniforth::forth;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("input")?;
    val.set_text_content(None);
    let button = document.create_element("button")?;
    button.set_text_content(Some("Run!"));

    body.append_child(&val)?;
    body.append_child(&button)?;

    let mut vm = miniforth::forth::VM::new(std::io::stdout());

    let value = val.dyn_into::<HtmlInputElement>().unwrap();

    let run = Closure::wrap(Box::new(move |_: Event| {
        document.create_element("button");
        console_log!("Hey!");
        console_log!("Value: {}", value.input());
        greet(value.input().as_str());
    }) as Box<dyn FnMut(_)>);

    button.add_event_listener_with_callback("click", &run.as_ref().unchecked_ref())?;

    run.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn fn_test() {
    greet("Works");
}

#[wasm_bindgen]
pub fn run_words() -> Result<String, JsValue> {
    Ok(String::new())
}
