use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("should have a document on window")
}

#[derive(Clone)]
pub struct View {
    tbl: Option<web_sys::Element>,
    cells: Vec<web_sys::Element>,
    timer: i32,
}

impl View {
    pub fn timeout(&mut self, callback: &Closure<dyn FnMut()>, ms: i32) {
        self.timer = window().set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), ms).unwrap();
    }

    pub fn cell(&self, i: usize) -> &web_sys::Element {
        &self.cells[i]
    }

    pub fn create(&mut self, m: usize, n: usize) {
        self.cells = vec![];
        self.tbl = document().get_element_by_id("tbl");
        for i in 0..m {
            let row = document().create_element("tr").unwrap();
            for j in 0..n {
                self.cells[i * n + j] = document().create_element("td").unwrap();
                if i == 1 {
                    // self.cells[i * n + j].set_inner_html(&arr[j].to_string());
                    self.cells[i * n + j].set_attribute("style", "2px solid");
                }
                row.append_child(&self.cells[i * n + j]);
            }
            self.tbl.unwrap().append_child(&row);
        }
    }

    pub fn clear(&self) {
        window().clear_timeout_with_handle(self.timer);
        self.tbl.unwrap().set_inner_html("");
    }
}
