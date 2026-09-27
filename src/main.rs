pub mod sorts;
pub mod state_info;
pub mod styles;
pub mod helpers;

use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::input::{KeyCode};

use crate::sorts::base_sort::SortType;
use crate::state_info::program_state::{SLOW_SPEED, MEDIUM_SPEED, HIGH_SPEED};
use crate::state_info::program_state::ListType;
use crate::state_info::program_state::ProgramState;
use crate::state_info::program_state::WatchSortsState;
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

    let all_sorts: Vec<SortType> = vec![
        SortType::Selection,
        SortType::Insertion,
        SortType::Bubble,
        SortType::CocktailShaker,
    ];

    let mut watch_sorts_state = WatchSortsState::new(
        ListType::FullyRandom,
        256,
        &all_sorts,
        HIGH_SPEED,
    );

    let ui_skin = default_style::default_style();
    root_ui().push_skin(&ui_skin);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        watch_sorts_state.control();
        watch_sorts_state.draw();

        /* match currentState {
            ProgramState::SelectListType => {
                clear_background(BLACK);
                
                widgets::Window::new(hash!(),
                    vec2(screen_width() / 2.0 - 200.0, screen_height() / 2.0 - 50.0),
                    vec2(400.0, 100.0),
                )
                .titlebar(true)
                .label("Select starting list type")
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "Select starting list type:");

                    if ui.button(None, "FULLY RANDOM") {
                        currentState = ProgramState::SelectSorts(ListType::FullyRandom);
                        for i in 0..all_sorts.len() {
                            selected_sorts[i] = false;
                        }
                    }

                    ui.same_line(0.0);
                    if ui.button(None, "SLIGHTLY RANDOM") {
                        currentState = ProgramState::SelectSorts(ListType::SlightlyRandom);
                        for i in 0..all_sorts.len() {
                            selected_sorts[i] = false;
                        }
                    }

                    ui.same_line(0.0);
                    if ui.button(None, "REVERSED") {
                        currentState = ProgramState::SelectSorts(ListType::Reversed);
                        for i in 0..all_sorts.len() {
                            selected_sorts[i] = false;
                        }
                    }
                });
                
            }
            ProgramState::SelectSorts(list_type) => {

            }
            ProgramState::WatchSorts => {
                
            }
        } */

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