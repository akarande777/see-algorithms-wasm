use wasm_bindgen::prelude::*;
use super::view::View;

static delay: i32 = 800;

struct BubbleSort {
    arr: Vec<i16>,
    n: usize,
    i: usize,
    j: usize,
    swapped: bool,
    view: View,
}

impl BubbleSort {
    pub fn iloop(&'static mut self) {
        let BubbleSort { arr, n, i, j, swapped, view } = *self;
        if i < n - 1 && swapped {
            swapped = false;
            j = 0;
            let jloop = Closure::wrap(Box::new(move || self.jloop()) as Box<dyn FnMut()>);
            view.timeout(&jloop, delay);
        } else {
            for k in 0..n {
                view.cell(k + n).set_attribute("bgcolor", "green");
            }
        }
    }

    pub fn jloop(&'static mut self) {
        let BubbleSort { arr, n, i, j, swapped, view } = *self;
        if j < n - i - 1 {
            view.cell(j + n - 1).remove_attribute("bgcolor");
            view.cell(j + n).set_attribute("bgcolor", "orange");
            view.cell(j + n + 1).set_attribute("bgcolor", "orange");
            if arr[j + 1] < arr[j] {
                let swap = Closure::wrap(Box::new(move || self.swap()) as Box<dyn FnMut()>);
                view.timeout(&swap, delay);
                swapped = true;
            } else {
                let jloop = Closure::wrap(Box::new(move || self.jloop()) as Box<dyn FnMut()>);
                view.timeout(&jloop, delay);
                j += 1;
            }
        } else {
            view.cell(j + n - 1).remove_attribute("bgcolor");
            view.cell(j + n).set_attribute("bgcolor", "orange");
            self.iloop();
            i += 1;
        }
    }

    fn swap(&'static mut self) {
        let BubbleSort { arr, n, i, j, swapped, view } = *self;
        let npn = n + n;
        let temp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = temp;
        view.timeout(
            &Closure::wrap(Box::new(move || {
                self.shift(j + 1, j + n + 1);
                self.shift(j + npn, j + n);
            }) as Box<dyn FnMut()>),
            150,
        );
        view.timeout(
            &Closure::wrap(Box::new(move || {
                self.shift(j, j + 1);
                self.shift(j + npn + 1, j + npn);
            }) as Box<dyn FnMut()>),
            300,
        );
        view.timeout(
            &Closure::wrap(Box::new(move || {
                self.shift(j + n, j);
                self.shift(j + n + 1, j + npn + 1);
                j += 1;
            }) as Box<dyn FnMut()>),
            450,
        );
        let jloop = Closure::wrap(Box::new(move || self.jloop()) as Box<dyn FnMut()>);
        view.timeout(&jloop, delay);
    }
    
    fn shift(&self, u: usize, v: usize) {
        let view = &self.view;
        view.cell(u).set_inner_html(&view.cell(v).inner_html());
        view.cell(u).set_attribute("bgcolor", "orange");
        view.cell(v).remove_attribute("bgcolor");
        view.cell(v).set_inner_html("");
    }

    fn start(&'static mut self, values: JsValue) {
        let BubbleSort { arr, n, i, j, swapped, view } = *self;
        arr = values.into_serde().unwrap();
        n = arr.len();
        view.create(3, n);
        for i in 0..n {
            view.cell(n + i).set_inner_html(&arr[i].to_string());
        }
        i = 0;
        j = 0;
        swapped = true;
        self.iloop();
    }

    fn stop(&self) {
        self.view.clear();
    }
}
