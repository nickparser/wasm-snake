use wasm_bindgen::{closure::Closure, closure::WasmClosure, JsCast};

pub struct Web;

impl Web {
    pub fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    pub fn document(window: &web_sys::Window) -> web_sys::Document {
        window.document().expect("should have a document on window")
    }

    pub fn canvas(document: &web_sys::Document, id: &str) -> web_sys::HtmlCanvasElement {
        let canvas = document
            .get_element_by_id(id)
            .expect(&format!("should have canvas by given id: {}", id)[..]);
        canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap()
    }

    pub fn context_2d(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }

    pub fn set_listener<T: WasmClosure + ?Sized>(
        document: &web_sys::Document,
        name: &str,
        listener: Closure<T>,
    ) {
        document
            .add_event_listener_with_callback(name, listener.as_ref().unchecked_ref())
            .expect("Listener binding failed!");
        listener.forget();
    }
}
