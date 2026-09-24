use macroquad::prelude::*;
use macroquad::ui::*;

pub fn default_style() -> Skin {
    let window_style = root_ui()
        .style_builder()
        .color(BLACK)
        .text_color(WHITE)
        .margin(RectOffset::new(10.0, 10.0, 10.0, 10.0))
        .font_size(20)
        .build();

    let button_style = root_ui()
        .style_builder()
        .margin(RectOffset::new(10.0, 10.0, 5.0, 5.0))
        .color(Color { r: 0.5, g: 1.0, b: 1.0, a: 1.0 })
        .color_hovered(Color { r: 0.5, g: 1.0, b: 1.0, a: 0.5 })
        .font_size(20)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font_size(50)
        .text_color(WHITE)
        .build();

    Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}