#![allow(unused)]
use egui::Color32;

pub struct KolorWheel {
    count: u32,
    h: f32, s: f32, l: f32,
    r: u8, g: u8, b: u8,
}

impl KolorWheel {

    pub fn new() -> KolorWheel {
        Self {
            count: 1,
            h: 0.5, s: 0.5, l: 0.5,
            r: 127, g: 127, b: 127,
        }
    }

    pub fn set_count(mut self, count: u32) -> KolorWheel {
        self.count = count;
        return self;
    }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> KolorWheel {

        self.r = r;
        self.g = g;
        self.b = b;

        return self;
    }


    pub fn set_color32(mut self, color: Color32) -> KolorWheel {
        return self.set_rgb(color.r(), color.g(), color.b());
    }
}

impl Iterator for KolorWheel {
    type Item = Color32;

    fn next(&mut self) -> Option<Color32>{

        if self.count == 0 {
            return None;
        }
        
        self.count -= 1;
        let color32 = Color32::DEBUG_COLOR;
        return Some(color32);
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
