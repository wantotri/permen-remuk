use std::{cell::RefCell, ops::Deref};
use std::cmp::max;
use std::fmt::Display;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

#[derive(Debug, PartialEq, Clone)]
pub enum Warna {
    Merah,
    Jingga,
    Kuning,
    Hijau,
    Biru,
    Nila,
    Ungu,
}

impl Distribution<Warna> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Warna {
        match rng.gen_range(0..7) {
            0 => Warna::Merah,
            1 => Warna::Jingga,
            2 => Warna::Kuning,
            3 => Warna::Hijau,
            4 => Warna::Biru,
            5 => Warna::Nila,
            _ => Warna::Ungu,
        }
    }
}

impl Display for Warna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Warna::Merah => write!(f, "🟥"),
            Warna::Jingga => write!(f, "🟧"),
            Warna::Kuning => write!(f, "🟨"),
            Warna::Hijau => write!(f, "🟩"),
            Warna::Biru => write!(f, "🟦"),
            Warna::Nila => write!(f, "🟫"),
            Warna::Ungu => write!(f, "🟪"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Permen {
    pub warna: RefCell<Warna>,
}

impl Display for Permen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.warna.borrow())
    }
}

impl Permen {
    pub fn new(warna: Warna) -> Permen {
        Permen { warna: RefCell::new(warna) }
    }

    /// Mengambil warna permen
    ///
    /// ### Example
    ///
    /// ```
    /// use crate::permen_remuk::permenremuk::{Permen, Warna};
    ///
    /// let p = Permen::new(Warna::Merah);
    ///
    /// assert_eq!(Warna::Merah, p.get_warna());
    /// ```
    pub fn get_warna(&self) -> Warna {
        self.warna.borrow().clone()
    }

    /// Menentukan warna untuk permen
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::permenremuk::{Permen, Warna};
    ///
    /// let p = Permen::new(Warna::Merah);
    /// p.set_warna(Warna::Kuning);
    ///
    /// assert_eq!(Warna::Kuning, p.get_warna());
    /// ```
    pub fn set_warna(&self, warna: Warna) {
        *self.warna.borrow_mut() = warna;
    }

    /// Mengacak warna yang sebelumnya sudah ditentukan
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::permenremuk::{Permen, Warna};
    ///
    /// let p = Permen::new(Warna::Merah);
    /// p.acak_warna();
    ///
    /// assert_ne!(*p.warna.borrow(), Warna::Merah);
    /// ```
    pub fn acak_warna(&self) {
        let warna_awal = self.get_warna();
        let mut warna_akhir = self.get_warna();

        while warna_awal == warna_akhir {
            self.set_warna(rand::random());
            warna_akhir = self.get_warna();
        }
    }
}

#[derive(Debug)]
pub struct Dempet {
    pub vektor: Vec<Posisi>
}

impl Deref for Dempet {
    type Target = Vec<Posisi>;

    fn deref(&self) -> &Self::Target {
        &self.vektor
    }
}

impl Dempet {
    pub fn new() -> Dempet {
        Dempet { vektor: vec![] }
    }

    /// Menambahkan Posisi ke dalam vektor Dempet.
    /// Jika pos sudah ada di dalam vektor Dempet,
    /// tidak ada yang ditambahkan
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::permenremuk::Dempet;
    ///
    /// let mut dempet = Dempet::new();
    /// dempet.tambah((1,1)).tambah((1,1));
    ///
    /// assert_eq!(dempet.len(), 1);
    /// ```
    pub fn tambah(&mut self, pos: Posisi) -> &mut Self {
        if self.vektor.iter().any(|tup| *tup == pos) {
            return self;
        }
        self.vektor.push(pos);
        self
    }

    /// Menambahkan vektor Posisi ke dalam vektor Dempet
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::permenremuk::Dempet;
    ///
    /// let mut dempet = Dempet::new();
    /// dempet.tambah((1,1)).tambah_vek(vec![(1,1),(2,2)]);
    ///
    /// assert_eq!(dempet.len(), 2);
    pub fn tambah_vek(&mut self, vektor: Vec<Posisi>) -> &mut Self {
        for vek in vektor {
            self.tambah(vek);
        }
        self
    }

    /// Mengurutkan vektor Dempet (a,b) berdasarkan a
    pub fn urutkan(&mut self) -> &mut Self {
        self.vektor.sort_by_key(|tup| tup.0);
        self
    }

    /// Mengosongkan vektor Dempet
    pub fn kosongkan(&mut self) ->&mut Self {
        self.vektor = vec![];
        self
    }

    /// Panjang vektor Dempet
    pub fn len(&self) -> usize {
        self.vektor.len()
    }

    /// Mengambil mana saja kolom yang berubah
    pub fn kolom_berubah(&self) -> Vec<Posisi> {
        let mut maks_y = 0;
        let mut min_x = 100;
        let mut maks_x = 0;

        for vek in &self.vektor {
            if vek.0 > maks_y { maks_y = vek.0 }
            if vek.1 > maks_x { maks_x = vek.1 }
            if vek.1 < min_x { min_x = vek.1 }
        }

        (min_x..=maks_x).map(|x| (maks_y, x)).collect()
    }
}

const PAPAN_MIN: usize = 3;
const PAPAN_MAX: usize = 10;

type Isian = Vec<Vec<Permen>>;
type Posisi = (usize, usize);

#[derive(Debug)]
pub struct Papan {
    pub ukuran: usize,
    pub ragam_warna: usize,
    pub isi: Isian
}

impl Display for Papan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for baris in self.isi.iter() {
            for permen in baris {
                write!(f, "{} ", permen)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Papan {
    pub fn new(ukuran: usize, ragam_warna: usize) -> Papan {
        // Batasi ukuran papan
        let mut ukuran_papan: usize = max(ukuran, PAPAN_MIN);
        if ukuran > PAPAN_MAX { ukuran_papan = PAPAN_MAX };

        let isi = Papan::isian_baru(ukuran_papan);
        Papan { ukuran: ukuran_papan, ragam_warna, isi }
    }

    /// Membuat isian baru agar tidak ada permen yang dempet
    pub fn isian_baru(ukuran: usize) -> Isian {
        let isi: Isian =
            (0..ukuran).map(|_| {
                (0..ukuran).map(move |_| {
                    Permen::new(rand::random())
                }).collect()
            }).collect();

        for (y, vec_permen) in isi.iter().enumerate() {
            for (x, _permen) in vec_permen.iter().enumerate() {
                if x >= 2 {
                    loop {
                        if *isi[y][x-2].warna.borrow() != *isi[y][x-1].warna.borrow() { break }
                        if *isi[y][x-1].warna.borrow() != *isi[y][x].warna.borrow() { break }
                        isi[y][x].acak_warna();
                    }
                }
                if y >= 2 {
                    loop {
                        if *isi[y-2][x].warna.borrow() != *isi[y-1][x].warna.borrow() { break }
                        if *isi[y-1][x].warna.borrow() != *isi[y][x].warna.borrow() { break }
                        isi[y][x].acak_warna();
                    }
                }
            }
        }

        isi
    }

    /// Mengambil warna permen pada Posisi pos
    pub fn get_warna(&self, pos: Posisi) -> Warna {
        self.isi[pos.0][pos.1].get_warna()
    }

    /// Menentukan warna permen pada Posisi pos
    pub fn set_warna(&self, pos: Posisi, warna: Warna) {
        self.isi[pos.0][pos.1].set_warna(warna)
    }

    /// Menukar warna dua permen yang bersebelahan
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::permenremuk::Papan;
    ///
    /// let papan = Papan::new(3, 3);
    /// let warna_lama = papan.isi[0][0].warna.clone();
    /// papan.tukar((0,0), (0,1));
    /// let warna_baru = papan.isi[0][1].warna.clone();
    ///
    /// assert_eq!(warna_lama, warna_baru);
    /// ```
    pub fn tukar(&self, pos1: Posisi, pos2: Posisi) {
        // hanya permen yang bersebelahan yang bisa ditukar
        if (pos1.0 as i16 - pos2.0 as i16).abs() > 1 || (pos1.1 as i16 - pos2.1 as i16).abs() > 1 {
            return;
        }
        let p1 = self.isi[pos1.0][pos1.1].clone();
        let p2 = self.isi[pos2.0][pos2.1].clone();
        self.set_warna(pos1, p2.get_warna());
        self.set_warna(pos2, p1.get_warna());
    }

    /// Meremukan permen dengan mengganti permen dengan permen di atasnya
    pub fn remukan(&self, pos: Posisi) {
        let mut ps = pos.clone();
        while ps.0 != 0 {
            // ganti warna permen dengan warna permen di atas permen tersebut
            self.set_warna((ps.0, pos.1), self.get_warna((ps.0-1, pos.1)));
            ps.0 -= 1;
        }
        self.set_warna((0, pos.1), rand::random());
    }

    // Mengecek permen "dempet" horizontal
    fn cek_horizontal(&self, pa: Posisi, tk: usize) -> Vec<Posisi> {
        let mut temp = Vec::<Posisi>::new();
        for idx in 0..self.ukuran {
            if self.get_warna((pa.0, idx)) == self.get_warna(pa) {
                temp.push((pa.0, idx));
                continue;
            }
            if idx > pa.1 { break }
            if temp.len() != 0 && temp.len() < tk { temp = vec![] }
            else {
                if temp.iter().any(|tup| *tup == pa) { break }
                else {
                    temp = vec![];
                    continue;
                }
            }
        }
        if temp.len() < tk { temp = vec![] }
        temp
    }

    // Mengecek permen "dempet" vertikal
    fn cek_vertikal(&self, pa: Posisi, tk: usize) -> Vec<Posisi> {
        let mut temp = Vec::<Posisi>::new();
        for idx in 0..self.ukuran {
            if self.get_warna((idx, pa.1)) == self.get_warna(pa) {
                temp.push((idx, pa.1));
                continue;
            }
            if idx > pa.0 { break }
            if temp.len() != 0 && temp.len() < tk { temp = vec![] }
            else {
                if temp.iter().any(|tup| *tup == pa) { break }
                else {
                    temp = vec![];
                    continue;
                }
            }
        }
        if temp.len() < tk { temp = vec![] }
        temp
    }

    /// Mengecek permen "dempet"
    /// (>=3 permen dengan warna yang sama saling besebelahan)
    pub fn cek_dempet(&self, pa: Posisi, tk: usize) -> Vec<Posisi> {
        // pa -> posisi awal | tk -> total kembar | tw -> target warna | pd -> posisi dempet
        let temp_x = self.cek_horizontal(pa, tk);
        let temp_y = self.cek_vertikal(pa, tk);

        let mut pd = Vec::<Posisi>::new();
        let mut temp_xy = temp_x;
        temp_xy.extend(temp_y);
        if temp_xy.len() > 0 {
            for pos in temp_xy {
                if pos.0 == pa.0 && pos.1 == pa.1 { continue }
                pd.push(pos);
            }
            pd.push(pa);
        }

        pd
    }

    /// Mengecek apakah permen dapat ditukar
    pub fn mungkin_ditukar(&self, pos: Posisi) -> bool {
        let mut mungkin = false;

        let tukar_dan_cek = |y, x| {
            let y0 = pos.0 as i16 + y;
            let x0 = pos.1 as i16 + x;
            let pos_target = (y0 as usize, x0 as usize);

            self.tukar(pos, pos_target);
            if self.cek_dempet(pos_target, 3).len() > 0 {
                self.tukar(pos, pos_target);
                return true
            }
            self.tukar(pos, pos_target);

            false
        };

        if pos.0 > 0 {
            mungkin = tukar_dan_cek(-1, 0);
            if mungkin { return mungkin }
        }
        if pos.0 + 1 < self.ukuran {
            mungkin = tukar_dan_cek(1, 0) ;
            if mungkin { return mungkin }
        }
        if pos.1 > 0 {
            mungkin = tukar_dan_cek(0, -1);
            if mungkin { return mungkin }
        }
        if pos.1 + 1 < self.ukuran {
            mungkin = tukar_dan_cek(0, 1);
            if mungkin { return mungkin }
        }

        mungkin
    }

    /// Mengembalikan vektor posisi permen yang dapat ditukar
    pub fn cek_kemungkinan(&self) -> Vec<Posisi> {
        let mut kemungkinan = Vec::new();
        for (y, baris) in self.isi.iter().enumerate() {
            for (x, _cell) in baris.iter().enumerate() {
                if self.mungkin_ditukar((y, x)) {
                    kemungkinan.push((y, x))
                }
            }
        }
        kemungkinan
    }

    pub fn cek_kemungkinan_str(&self) -> String {
        let mut result = String::new();
        let vektor_kemungkinan = self.cek_kemungkinan();
        for pos in vektor_kemungkinan {
            result.push_str(&format!("{}:{} ", pos.0, pos.1))
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vektor_warna() -> Vec<Warna> {
        vec![
            Warna::Merah,
            Warna::Jingga,
            Warna::Kuning,
            Warna::Hijau,
            Warna::Biru,
            Warna::Nila,
            Warna::Ungu,
        ]
    }

    #[test]
    fn permen_new() {
        let permen = Permen::new(Warna::Biru);
        assert_eq!(*permen.warna.borrow(), Warna::Biru);
    }

    #[test]
    fn permen_get_warna() {
        let permen = Permen::new(Warna::Biru);
        assert_eq!(permen.get_warna(), Warna::Biru);
    }

    #[test]
    fn permen_set_warna() {
        let permen = Permen::new(Warna::Biru);
        permen.set_warna(Warna::Merah);
        assert_eq!(permen.get_warna(), Warna::Merah);
    }

    #[test]
    fn permen_acak_warna() {
        let permen = Permen::new(Warna::Merah);
        permen.acak_warna();

        for _ in 0..100 {
            assert_ne!(permen.get_warna(), Warna::Merah);
        }
    }

    #[test]
    fn dempet_new() {
        let dempet = Dempet::new();
        assert_eq!(dempet.vektor, vec![]);
    }

    #[test]
    fn dempet_tambah() {
        let mut dempet = Dempet::new();
        dempet.tambah((0, 0)).tambah((1, 1));
        assert_eq!(dempet.len(), 2);
        dempet.tambah((0,0));
        assert_eq!(dempet.len(), 2);
    }

    #[test]
    fn dempet_kosongkan() {
        let mut dempet = Dempet::new();
        dempet.tambah((0, 0)).tambah((1, 1));
        dempet.kosongkan();
        assert_eq!(dempet.len(), 0);
    }

    #[test]
    fn dempet_tambah_vektor() {
        let mut dempet = Dempet::new();
        dempet.tambah_vek(vec![(1, 0), (3, 0), (2, 0)]);
        assert_eq!(dempet.len(), 3);
    }

    #[test]
    fn dempet_urutkan() {
        let mut dempet = Dempet::new();
        dempet.tambah_vek(vec![(3, 0), (1, 0), (2, 0)]);
        dempet.urutkan();
        assert_eq!(dempet.vektor, vec![(1, 0), (2, 0), (3, 0)]);
    }

    #[test]
    fn dempet_kolom_berubah() {
        let mut dempet = Dempet::new();
        dempet.tambah_vek(vec![(3, 0), (1, 0), (2, 0)]);
        dempet.tambah_vek(vec![(1, 0), (1, 1), (1, 2)]);
        dempet.urutkan();
        let kolom_berubah = dempet.kolom_berubah();
        assert_eq!(kolom_berubah, vec![(3, 0), (3, 1), (3, 2)]);
    }

    #[test]
    fn papan_new() {
        let papan = Papan::new(5, 3);
        assert_eq!(papan.isi.len(), 5);
        assert_eq!(papan.isi[0].len(), 5);

        // assert isi dari papan adalah Warna
        let vektor_warna = vektor_warna();

        for baris in papan.isi.iter() {
            for permen in baris.iter() {
                assert!(vektor_warna.iter().any(|warna| {
                    permen.get_warna() == *warna
                }));
            }
        }
    }

    #[test]
    fn papan_isian_baru() {
        let isian = Papan::isian_baru(3);

        // assert isi dari isian adalah Warna
        let vektor_warna = vektor_warna();

        for baris in isian.iter() {
            for permen in baris.iter() {
                assert!(vektor_warna.iter().any(|warna| {
                    permen.get_warna() == *warna
                }));
            }
        }

        // assert tidak mungkin ada dempet (3 warna yang sama bersebelahan)
        for _ in 0..100 {
            let isian = Papan::isian_baru(3);
            let warna0 = isian[0][0].get_warna();
            let warna1 = isian[0][1].get_warna();
            let warna2 = isian[0][2].get_warna();
            assert!( !(warna0 == warna1 && warna1 == warna2) );
        }
    }

    #[test]
    fn papan_get_warna() {
        let papan = Papan::new(5, 3);
        let warna = papan.isi[0][0].warna.borrow().clone();
        assert_eq!(papan.get_warna((0, 0)), warna);
    }

    #[test]
    fn papan_set_warna() {
        let papan = Papan::new(5, 3);
        papan.set_warna((0, 0), Warna::Merah);
        assert_eq!(papan.get_warna((0, 0)), Warna::Merah);
    }

    #[test]
    fn papan_tukar() {
        let papan = Papan::new(3, 3);
        let warna0 = papan.get_warna((0, 0));
        papan.tukar((0,0), (0,1));
        let warna1 = papan.get_warna((0, 1));
        assert_eq!(warna0, warna1);
    }

    #[test]
    fn papan_remukan() {
        let papan = Papan::new(3, 3);
        let warna0 = papan.get_warna((0, 0));
        let warna1 = papan.get_warna((1, 0));
        papan.remukan((2, 0));
        assert_eq!(papan.get_warna((2, 0)), warna1);
        assert_eq!(papan.get_warna((1, 0)), warna0);
    }

    #[test]
    fn papan_cek_horizontal() {
        let papan = Papan::new(7, 3);
        for i in 0..papan.ukuran {
            papan.set_warna((0, i), Warna::Merah);
        }
        let dempet = papan.cek_horizontal((0, 0), 3);
        assert_eq!(dempet.len(), 7);

        papan.set_warna((0, 3), Warna::Biru);
        let dempet = papan.cek_horizontal((0, 0), 3);
        assert_eq!(dempet.len(), 3);
        let dempet = papan.cek_horizontal((0, 6), 3);
        assert_eq!(dempet.len(), 3);
    }

    #[test]
    fn papan_cek_vertikal() {
        let papan = Papan::new(7, 3);
        for i in 0..papan.ukuran {
            papan.set_warna((i, 0), Warna::Merah);
        }
        let dempet = papan.cek_vertikal((0, 0), 3);
        assert_eq!(dempet.len(), 7);

        papan.set_warna((3, 0), Warna::Biru);
        let dempet = papan.cek_vertikal((0, 0), 3);
        assert_eq!(dempet.len(), 3);
        let dempet = papan.cek_vertikal((6, 0), 3);
        assert_eq!(dempet.len(), 3);
    }

    #[test]
    fn papan_cek_dempet() {
        let papan = Papan::new(7, 3);
        for i in 0..papan.ukuran {
            papan.set_warna((i, 0), Warna::Merah);
            papan.set_warna((0, i), Warna::Merah);
        }
        let dempet = papan.cek_dempet((0, 0), 3);
        assert_eq!(dempet.len(), 13);
    }

    #[test]
    fn papan_mungkin_ditukar() {
        let papan = Papan::new(5, 3);
        papan.set_warna((0, 0), Warna::Merah);
        papan.set_warna((1, 1), Warna::Merah);
        papan.set_warna((1, 2), Warna::Merah);
        assert!(papan.mungkin_ditukar((0, 0)));
    }

    #[test]
    fn papan_cek_kemungkinan() {
        let papan = Papan::new(5, 3);
        papan.set_warna((0, 0), Warna::Merah);
        papan.set_warna((1, 1), Warna::Merah);
        papan.set_warna((1, 2), Warna::Merah);
        assert!(papan.cek_kemungkinan().len() >= 1);
    }

    #[test]
    fn papan_cek_kemungkinan_str() {
        let papan = Papan::new(5, 3);
        let mut result = String::new();
        let vektor_kemungkinan = papan.cek_kemungkinan();
        for pos in vektor_kemungkinan {
            result.push_str(&format!("{}:{} ", pos.0, pos.1))
        }
        assert_eq!(papan.cek_kemungkinan_str(), result);
    }
}
