#![allow(unused)]
use egui::Color32;

pub struct KolorWheel {
    base_color: Color32,
    count: u32,
}

impl KolorWheel {

    pub fn new() -> KolorWheel {
        KolorWheel {
            base_color: Color32::DEBUG_COLOR,
            count: 1,
        }
    }

    pub fn set_count(mut self, count: u32) -> KolorWheel {
        self.count = count;
        return self;
    }
}

impl Iterator for KolorWheel {
    type Item = Color32;

    fn next(&mut self) -> Option<Color32>{

        if self.count == 0 {
            return None;
        }
        
        self.count -= 1;
        return Some(self.base_color);
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
