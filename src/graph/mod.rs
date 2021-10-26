mod segment;

use segment::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

#[wasm_bindgen]
pub struct Graph {
    points: Vec<Point>,
    segments: Vec<Segment>,
    matrix: Vec<Vec<Option<usize>>>,
    steps: Vec<(usize, usize)>,
    directed: bool,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        Graph {
            points: vec![],
            segments: vec![],
            matrix: vec![],
            steps: vec![],
            directed: false,
        }
    }

    pub fn addPoint(&mut self, p: &Point) {
        self.points.push(p.clone());
        let size = self.points.len();
        self.matrix.push(vec![None; size]);
        for i in 0..(size - 1) {
            self.matrix[i].push(None);
        }
    }

    pub fn setPoint(&mut self, i: usize, p: &Point) {
        self.points[i] = p.clone();
    }

    pub fn position(&self, s: &Segment) -> JsValue {
        let i = self.points.iter().position(|&p| p.equals(&s.p)).unwrap();
        let j = self.points.iter().position(|&p| p.equals(&s.q)).unwrap();
        JsValue::from_serde(&(i, j)).unwrap()
    }

    pub fn addSegment(&mut self, s: &Segment) {
        let (i, j): (usize, usize) = self.position(s).into_serde().unwrap();
        let size = self.segments.len();
        self.matrix[i][j] = Some(size);
        if !self.directed {
            self.matrix[j][i] = Some(size);
        }
        self.segments.push(s.clone());
        self.steps.push((i, j));
    }

    pub fn totalPoints(&self) -> usize {
        self.points.len()
    }

    pub fn totalSegments(&self) -> usize {
        self.segments.len()
    }

    pub fn point(&self, i: usize) -> Point {
        self.points[i]
    }

    pub fn segment(&self, i: usize) -> Segment {
        self.segments[i]
    }

    pub fn clear(&mut self) {
        self.points = vec![];
        self.segments = vec![];
        self.matrix = vec![];
        self.steps = vec![];
    }

    pub fn edgeIndex(&self, i: usize, j: usize) -> isize {
        let value = self.matrix[i][j];
        if value.is_some() {
            self.matrix[i][j].unwrap() as isize
        } else {
            -1
        }
    }

    pub fn forEach(&self, callback: &js_sys::Function) {
        let size = self.points.len();
        for i in 0..size {
            for j in 0..size {
                callback.call2(
                    &JsValue::null(),
                    &JsValue::from_f64(i as f64),
                    &JsValue::from_f64(j as f64),
                );
            }
        }
    }

    pub fn isDirected(&self) -> bool {
        self.directed
    }

    pub fn isConnected(&self) -> bool {
        let visited = self.dfs(0, &mut vec![0]);
        visited.len() == self.points.len()
    }

    fn dfs(&self, u: usize, visited: &mut Vec<usize>) -> Vec<usize> {
        let size = self.points.len();
        for v in 0..size {
            let index = visited.iter().position(|&i| i == v);
            if index == None {
                let ui = self.matrix[u][v];
                let vi = self.matrix[v][u];
                if ui != None || vi != None {
                    visited.push(v);
                    self.dfs(v, visited);
                }
            }
        }
        visited.clone()
    }

    pub fn switchType(&mut self) {
        self.directed = !self.directed;
        if self.points.len() > 1 {
            if self.directed {
                for &(i, j) in self.steps.iter() {
                    self.matrix[j][i] = None;
                }
            } else {
                for &(i, j) in self.steps.iter() {
                    self.matrix[j][i] = self.matrix[i][j];
                }
            }
        }
    }

    pub fn removeSegment(&mut self, s: &Segment) {
        let (i, j): (usize, usize) = self.position(s).into_serde().unwrap();
        self.segments.remove(self.matrix[i][j].unwrap());
        self.matrix[i][j] = None;
        if !self.directed {
            self.matrix[j][i] = None;
        }
        let k = self.steps.iter().position(|&(u, v)| u == i && v == j);
        self.steps.remove(k.unwrap());
    }

    pub fn indegree(&self) -> JsValue {
        let size = self.points.len();
        let mut ind = vec![0; size];
        for &step in self.steps.iter() {
            ind[step.1] += 1;
        }
        JsValue::from_serde(&ind).unwrap()
    }

    pub fn hasCycle(&self) -> bool {
        let size = self.points.len();
        let mut ind: Vec<usize> = self.indegree().into_serde().unwrap();
        let mut stack = vec![];
        for i in 0..size {
            if ind[i] == 0 {
                stack.push(i);
            }
        }
        if stack.len() == 0 {
            return true;
        }
        let mut k = 0;
        while stack.len() > 0 {
            let i = stack.pop().unwrap();
            for j in 0..size {
                let ei = self.edgeIndex(i, j);
                if ei != -1 && ind[j] != 0 {
                    ind[j] -= 1;
                    if ind[j] == 0 {
                        stack.push(j);
                    }
                }
            }
            k += 1;
        }
        k != size
    }

    #[wasm_bindgen(getter)]
    pub fn points(&self) -> JsValue {
        JsValue::from_serde(&self.points).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn matrix(&self) -> JsValue {
        JsValue::from_serde(&self.matrix).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn segments(&self) -> JsValue {
        JsValue::from_serde(&self.segments).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> JsValue {
        JsValue::from_serde(&self.steps).unwrap()
    }
}
