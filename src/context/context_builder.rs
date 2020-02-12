use std::string::ToString;

use rltk::{Console, console, Rltk, SimpleConsole, SparseConsole};

use crate::{CONSOLE_INDEX, LAYER_COUNT, TITLE};

const LAYER_OFFSET_X: f32 = 0.06;
const LAYER_OFFSET_Y: f32 = 0.12;
const LAYER_STATIC_OFFSET_X: f32 = 0.;
const LAYER_STATIC_OFFSET_Y: f32 = 0.;
const SHADER_PATH: &str = "resources";
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

pub struct ContextBuilder<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
}

pub fn build_context(width: i32, height: i32, title: &str) -> Rltk {
    ContextBuilder {
        width: width as u32,
        height: height as u32,
        title,
    }.create_context()
}

impl<'a> ContextBuilder<'a> {
    pub fn create_context(&self) -> Rltk {
        let mut context = Rltk::init_raw(self.width * TILE_WIDTH, self.height * TILE_HEIGHT, TITLE);

        let mut p_w = TILE_WIDTH;
        let mut p_h = TILE_HEIGHT;

        let base_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                has_bg: true,
                tile_width: Some(p_w),
                tile_height: Some(p_h),
                ..Default::default()
            },
        );

        p_w += 1;
        p_h += 1;

        check_console_index(CONSOLE_INDEX.base, base_console_index);

        for layer in 0..LAYER_COUNT {
            let offset_multiplier = (layer + 1) as f32;

            let layer_console_index = self.add_console(
                &mut context,
                AddConsoleParameter {
                    offset_x: LAYER_STATIC_OFFSET_X + LAYER_OFFSET_X * offset_multiplier,
                    offset_y: LAYER_STATIC_OFFSET_Y + LAYER_OFFSET_Y * offset_multiplier,

                    tile_width: Some(p_w),
                    tile_height: Some(p_h),
                    is_sparse: true,
                    ..Default::default()
                });

            p_w += 1;
            p_h += 1;

            check_console_index(CONSOLE_INDEX.layers[layer], layer_console_index);
        }

        let ui_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                is_sparse: true,
                has_bg: true,

                tile_width: Some(p_w),
                tile_height: Some(p_h),
                ..Default::default()
            });

        check_console_index(CONSOLE_INDEX.ui, ui_console_index);

        context.set_active_console(CONSOLE_INDEX.base);
        context.with_post_scanlines(false);

        context
    }

    fn add_console(&self, context: &mut Rltk, params: AddConsoleParameter) -> usize {
        let tile_width = params.tile_width.unwrap_or(TILE_WIDTH);
        let tile_height = params.tile_width.unwrap_or(TILE_HEIGHT);
        let w_ratio = tile_width as f32 / TILE_WIDTH as f32;
        let h_ratio = tile_height as f32 / TILE_HEIGHT as f32;

        console::log(format!("{}, {}", tile_width, tile_height));

        let font_path = format!("{}/terminal8x8.png", &SHADER_PATH.to_string());
        let font = context.register_font(rltk::Font::load(font_path, (tile_width, tile_height)));

        let mut console: Box<dyn Console>;

        let w = (self.width as f32 * w_ratio) as u32;
        let h = (self.height as f32 * h_ratio) as u32;

        if params.is_sparse {
            console = SparseConsole::init(w, h, &context.backend);
        } else {
            console = SimpleConsole::init(w, h, &context.backend);
        }

        console.set_offset(params.offset_x, params.offset_y);
        if params.has_bg {
            context.register_console(console, font)
        } else {
            context.register_console_no_bg(console, font)
        }
    }
}

#[derive(Default)]
struct AddConsoleParameter {
    pub offset_x: f32,
    pub offset_y: f32,
    pub is_sparse: bool,
    pub has_bg: bool,
    pub tile_width: Option<u32>,
    pub tile_height: Option<u32>,

}

fn check_console_index(expected: usize, actual: usize) {
    if expected != actual {
        panic!("Incorrect console index: expected {}, got {}", expected, actual);
    }
}
