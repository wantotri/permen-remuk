// #![allow(dead_code)]
pub mod permenremuk;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use permenremuk::*;

thread_local! {
    static PAPAN: Papan = Papan::new(6, 3);
    static DEMPET: RefCell<Dempet> = RefCell::new(Dempet::new());
}

#[wasm_bindgen(js_name = buatPapan)]
pub fn buat_papan() -> String {
    PAPAN.with(|ppn| ppn.to_string())
}

#[wasm_bindgen(js_name = tukarPosisi)]
pub fn tukar_posisi(y1: usize, x1: usize, y2: usize, x2: usize) {
    PAPAN.with(|ppn| ppn.tukar((y1, x1), (y2, x2)) );
}

#[wasm_bindgen(js_name = cekDempet)]
pub fn cek_dempet(y: usize, x: usize, tk: usize) {
    DEMPET.with(|dmpt| {
        dmpt.borrow_mut()
            .tambah_vek(
                PAPAN.with(|ppn| ppn.cek_dempet((y, x), tk)))
            .urutkan();
    });
}

#[wasm_bindgen(js_name = panjangDempet)]
pub fn panjang_dempet() -> usize {
    DEMPET.with(|dmpt| dmpt.borrow().len())
}

#[wasm_bindgen(js_name = remukanPermen)]
pub fn remukan_permen() {
    DEMPET.with(|dmpt| {
        for pos in dmpt.borrow().iter() {
            PAPAN.with(|ppn| ppn.remukan(*pos))
        }
        dmpt.borrow_mut().kosongkan();
    });
}

#[wasm_bindgen(js_name = bisaJalan)]
pub fn bisa_jalan() -> bool {
    PAPAN.with(|ppn| {
        if ppn.cek_kemungkinan().len() > 0 {
            return true
        }
        false
    })
}

#[wasm_bindgen(js_name = cekKemungkinan)]
pub fn cek_kemungkinan() -> String {
    PAPAN.with(|ppn| ppn.cek_kemungkinan_str())
}
