use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn equals(&self, q: &Point) -> bool {
        self.x == q.x && self.y == q.y
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Segment {
    pub p: Point,
    pub q: Point,
}

#[wasm_bindgen]
impl Segment {
    #[wasm_bindgen(constructor)]
    pub fn new(p: &Point, q: &Point) -> Segment {
        Segment {
            p: p.clone(),
            q: q.clone(),
        }
    }

    pub fn slope(&self) -> f32 {
        (self.q.y - self.p.y) / (self.q.x - self.p.x)
    }

    pub fn orientation(&self, r: &Point) -> u8 {
        let (p, q) = (self.p, self.q);
        let d = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if d == 0.0 {
            return 0;
        }
        return if d > 0.0 { 1 } else { 2 };
    }

    pub fn overlaps(&self, s: &Segment) -> bool {
        if (self.p.equals(&s.p) && self.q.equals(&s.q))
            || (self.p.equals(&s.q) && self.q.equals(&s.p))
        {
            return true;
        }
        return false;
    }
}
