use crate::vm::core::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

pub struct EmuWasm {
    chip8: Emulator,
    ctx: CanvasRenderingContext2d,
}

impl EmuWasm {
    pub fn new() -> EmuWasm {
        let chip8 = Emulator::new();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        EmuWasm { chip8, ctx }
    }

    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = key_to_btn(&key) {
            self.chip8.keypressed(k, pressed);
        }
    }

    pub fn load_game(&mut self, data: Uint8Array) {
        self.chip8.load(&data.to_vec());
    }

    pub fn draw_screen(&mut self, scale: usize) {
        let disp = self.chip8.get_display();
        let canvas_w = (SCREEN_WIDTH * scale) as f64;
        let canvas_h = (SCREEN_HEIGHT * scale) as f64;

        self.ctx.set_fill_style_str("black");
        self.ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);
        self.ctx.set_fill_style_str("magenta");

        for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
            if disp[i] {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }
}

fn key_to_btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
