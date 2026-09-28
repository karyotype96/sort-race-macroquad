pub mod sorts;
pub mod state_info;
pub mod styles;
pub mod helpers;

use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::input::{KeyCode};

use crate::sorts::base_sort::FRAME_RATE;
use crate::sorts::base_sort::SortType;
use crate::state_info::program_state::ListType;
use crate::state_info::program_state::ProgramState;
use crate::state_info::program_state::WatchSortsState;
use crate::styles::default_style;

pub struct SelectSortsState {
    list_type: usize,
    sorts: Vec<bool>,
    ops_per_second: f32,
    // the actual size of the list will be 2 raised to
    // the power of this value floored
    list_size_exponent: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sort Race".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_state = ProgramState::SelectSorts;

    let all_sorts: Vec<SortType> = vec![
        SortType::Selection,
        SortType::Insertion,
        SortType::Bubble,
        SortType::CocktailShaker,
        SortType::OutOfPlaceMerge,
        SortType::Comb,
    ];
    
    let mut select_sorts_state = SelectSortsState {
        list_type: 0,
        sorts: all_sorts.iter().map(|_| false).collect(),
        ops_per_second: 1.0,
        list_size_exponent: 4.0,
    };

    let list_type_options = &["Fully Random", "Slightly Random", "Reversed"];

    let mut watch_sorts_state = WatchSortsState::default();

    let ui_skin = default_style::default_style();
    root_ui().push_skin(&ui_skin);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let ops = select_sorts_state.ops_per_second;
        let actual_speed = FRAME_RATE / ops;

        match current_state {
            ProgramState::SelectSorts => {
                root_ui().window(hash!("starting list window"), 
                    vec2(50.0, 50.0),
                    vec2(screen_width() / 2.0 - 100.0, 200.0),
                    |ui| {
                        ui.combo_box(
                            hash!("starting list type"), 
                            "Starting List Type",
                            list_type_options,
                            &mut select_sorts_state.list_type
                        );
                    }
                );

                root_ui().window(hash!("selected sorts window"),
                    vec2(50.0, screen_height() / 4.0),
                    vec2(screen_width() / 2.0 - 100.0, screen_height() / 4.0),
                    |ui| {
                        ui.label(None, "Select sorts to show:");
                        for i in 0..all_sorts.len() {
                            let name = all_sorts[i].get_name();
                            ui.checkbox(hash!(name), 
                                name,
                                &mut select_sorts_state.sorts[i]
                            );
                        }
                    }
                );

                root_ui().window(hash!("speed slider window"),
                    vec2(screen_width() / 2.0 + 50.0, 50.0),
                    vec2(screen_width() / 2.0 - 100.0, screen_height() / 4.0),
                    |ui| {
                        ui.label(None, "Playback Speed");
                        ui.slider(hash!("speed slider"),
                            "",
                            std::ops::Range { start: 1.0, end: 10000.0 },
                            &mut select_sorts_state.ops_per_second,
                        );
                        ui.label(None, &format!("{} operations per second", ops.floor()))
                    }
                );

                root_ui().window(hash!("list size window"),
                    vec2(screen_width() / 2.0 + 50.0, screen_height() / 4.0 + 60.0),
                    vec2(screen_width() / 2.0 - 100.0, screen_height() / 4.0),
                    |ui| {
                        ui.label(None, "List Size");
                        ui.slider(hash!("list size slider"),
                            "",
                            std::ops::Range { start: 4.0, end: 10.0 },
                            &mut select_sorts_state.list_size_exponent
                        );
                        ui.label(None, &format!("List size: {} items", 2_i32.pow(select_sorts_state.list_size_exponent.floor() as u32)))
                    }
                );


                root_ui().window(hash!("ready button"),
                    vec2(screen_width() / 2.0 - 300.0, 3.0 * screen_height() / 4.0),
                    vec2(300.0, 200.0),
                    |ui| {
                        let selected_count = select_sorts_state.sorts
                            .iter()
                            .filter(|&sort| *sort)
                            .collect::<Vec<&bool>>().len();

                        if selected_count < 1 {
                            draw_text("Please select at least one sort.", 
                                screen_width() / 2.0 - 300.0, 
                                3.0 * screen_height() / 4.0, 
                                40.0, 
                                BLUE
                            );
                        } else if selected_count > 9 {
                            draw_text("Please select 9 sorts or less.", 
                                screen_width() / 2.0 - 300.0, 
                                3.0 * screen_height() / 4.0, 
                                40.0, 
                                BLUE
                            );
                        } else {
                            if ui.button(None, "Start sorting!") {
                                let mut selected_sorts = Vec::new();
                                for (i, s) in all_sorts.iter().enumerate() {
                                    if select_sorts_state.sorts[i] {
                                        selected_sorts.push(s);
                                    }
                                }

                                let list_type = match select_sorts_state.list_type {
                                    0 => ListType::FullyRandom,
                                    1 => ListType::SlightlyRandom,
                                    _ => ListType::Reversed,
                                };

                                watch_sorts_state = WatchSortsState::new(
                                    list_type,
                                    2_u32.pow(select_sorts_state.list_size_exponent.floor() as u32),
                                    &selected_sorts,
                                    actual_speed as f64
                                );

                                current_state = ProgramState::WatchSorts;
                            }
                        }
                    }
                );
            }
            ProgramState::WatchSorts => {
                watch_sorts_state.control();
                watch_sorts_state.draw();

                if is_key_pressed(KeyCode::Q) {
                    current_state = ProgramState::SelectSorts;
                }
            }
        }

        next_frame().await
    }
}