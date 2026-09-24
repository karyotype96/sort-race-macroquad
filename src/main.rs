pub mod sorts;
pub mod state_info;
pub mod styles;

use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::input::{KeyCode};

use crate::sorts::base_sort::Sort;
use crate::sorts::selection_sort::SelectionSort;
use crate::state_info::program_state::ProgramState;
use crate::styles::default_style;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sort Race".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut currentState = ProgramState::SelectListType;

    let mut data = vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6];
    let mut sort = SelectionSort::default();

    sort.init_sort(&data);
    println!("data: {:?}", sort.data);

    while !sort.is_finished() {
        sort.advance_sort();
        println!("data: {:?}", sort.data);
    }

    let ui_skin = default_style::default_style();

    root_ui().push_skin(&ui_skin);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        match currentState {
            ProgramState::SelectListType => {
                clear_background(BLACK);

                widgets::Window::new(hash!(),
                    vec2(screen_width() / 2.0 - 200.0, screen_height() / 2.0 - 10.0),
                    vec2(400.0, 70.0),
                )
                .titlebar(true)
                .label("Select starting list type")
                .ui(&mut root_ui(), |ui| {
                    if ui.button(None, "FULLY RANDOM") {
                        println!("FULLY RANDOM");
                    }

                    ui.same_line(0.0);
                    if ui.button(None, "SLIGHTLY RANDOM") {
                        println!("SLIGHTLY RANDOM");
                    }

                    ui.same_line(0.0);
                    if ui.button(None, "REVERSED") {
                        println!("REVERSED");
                    }
                });

                /* root_ui().window(
                    hash!(), 
                    vec2(screen_width() / 2.0 - 200.0, screen_height() / 2.0 - 30.0),
                    vec2(screen_width(), 50.0),
                    |ui| {

                        if ui.button(None, "Fully Random") {
                            println!("Fully Random");
                        }
                        ui.same_line(150.0);
                        if ui.button(None, "Slightly Random") {
                            println!("Slightly Random");
                        }
                        ui.same_line(325.0);
                        if ui.button(None, "Reversed") {
                            println!("Reversed");
                        }
                    }); */
                
            }
            ProgramState::SelectSorts(list_type) => {

            }
            ProgramState::WatchSorts => {
                
            }
        }

        next_frame().await
    }
}

// state views
fn select_list_type_view(currentState: &mut ProgramState) {

}

fn select_sorts_view(currentState: &mut ProgramState) {

}

fn watch_sorts(currentState: &mut ProgramState) {

}