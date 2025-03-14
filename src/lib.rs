use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

// Game state
#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Game, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Game { context, x: 50.0, y: 50.0 })
    }

    pub fn update(&mut self) {
        self.context.clear_rect(0.0, 0.0, 500.0, 500.0);
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context.fill_rect(self.x, self.y, 30.0, 30.0);
    }

    pub fn move_square(&mut self, direction: &str) {
        match direction {
            "ArrowUp" => self.y -= 5.0,
            "ArrowDown" => self.y += 5.0,
            "ArrowLeft" => self.x -= 5.0,
            "ArrowRight" => self.x += 5.0,
            _ => {}
        }
    }
}
