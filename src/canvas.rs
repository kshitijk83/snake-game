use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{console, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

// #[allow(dead_code)]
// // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/Window
// pub fn window() -> Result<Window> {
//     web_sys::window()
//         .ok_or_else(|| anyhow!("No Window Found"))
// }

// #[allow(dead_code)]
// // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/Document
// pub fn document() -> Result<Document> {
//     window()?
//         .document()
//         .ok_or_else(|| anyhow!("No Document Found"))
// }

// #[allow(dead_code)]
// // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/HtmlCanvasElement
// pub fn canvas(id_selector: &str) -> Result<HtmlCanvasElement> {
//     document()?
//         .get_element_by_id(id_selector)
//         .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
// }

// #[allow(dead_code)]
// // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2d
// pub fn canvas_context_2d(id_selector: &str) -> Result<CanvasRenderingContext2d> {
//     canvas(id_selector)?
//         .get_context("2d")
//         .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
//         .ok_or_else(|| anyhow!("No 2d context found"))?
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .map_err(|element| {
//             anyhow!(
//                 "Error converting {:#?} to CanvasRenderingContext2d",
//                 element
//             )
//         })
// }

pub struct Canvas{
    pub canvas: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_height: u32,
    scaled_width: u32,
    width: u32,
    height: u32
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) ->Canvas{
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(attr_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
    
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let scaled_height = canvas.height()/height;
        let scaled_width = canvas.height()/width;

        Canvas{
            canvas,
            ctx,
            width,
            height,
            scaled_height,
            scaled_width
        }
    }

    pub fn draw(&self, x:u32, y: u32, color: &str){
        assert!(x<self.width);
        assert!(y<self.height);

        self.ctx.set_fill_style(&JsValue::from_str(color));

        let x = x*self.scaled_width;
        let y = y*self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );

        // self.ctx.stroke();
    }

    pub fn clear_all(&self){
        self.ctx.set_fill_style(&JsValue::from("white"));
        self.ctx.fill_rect(0.0, 0.0, f64::from(self.width*self.scaled_width), f64::from(self.height*self.scaled_height));
    }
}