#![allow(unused)]
use egui::Color32;

pub struct KolorWheel {
    base_color: Color32,
    count: u32,
}

impl KolorWheel {

    pub fn new() -> KolorWheel {
        KolorWheel {
            base_color: Color32::BLACK,
            count: 1,
        }
    }

    pub fn set_count(&self, count: u32) -> &KolorWheel {
        return &self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_nothing() {
        assert_eq!(2, 2);
    }
}

pub struct Kolor {
    h: f32,
    s: f32,
    l: f32,
}
