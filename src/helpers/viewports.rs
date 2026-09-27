use macroquad::{math::Rect, window::{screen_width, screen_height}};

pub fn get_viewports(count: usize) -> Vec<Rect> {
    let aspect_ratio = screen_width() / screen_height();

    let w = screen_width();
    let h = screen_height();

    match count {
        1 => {
            vec![
                Rect::new(0.0, 0.0, w, h)
            ]
        }
        2 => {
            vec![
                Rect::new(0.0, h / 4.0, w / 2.0, h / 2.0),
                Rect::new(w / 2.0, h / 4.0, w / 2.0, h / 2.0)
            ]
        }
        3 => {
            vec![
                Rect::new(w / 4.0, h / 2.0, w / 2.0, h / 2.0),
                Rect::new(0.0, 0.0, w / 2.0, h / 2.0),
                Rect::new(w / 2.0, 0.0, w / 2.0, h / 2.0)
            ]
        }
        4 => {
            vec![
                Rect::new(0.0, h / 2.0, w / 2.0, h / 2.0),
                Rect::new(w / 2.0, h / 2.0, w / 2.0, h / 2.0),
                Rect::new(0.0, 0.0, w / 2.0, h / 2.0),
                Rect::new(w / 2.0, 0.0, w / 2.0, h / 2.0)
            ]
        }
        _ => {
            panic!("Should not be able to select more than 9 sorts")
        }
    }
}