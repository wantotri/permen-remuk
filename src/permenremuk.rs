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

    /// Mengacak warna yang sebelumnya sudah ditentukan
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::{Permen,Warna};
    ///
    /// let p = Permen::new(Warna::Merah);
    /// p.acak_warna();
    ///
    /// assert_ne!(*p.warna.borrow(), Warna::Merah);
    /// ```
    pub fn acak_warna(&self) {
        let warna_awal = self.warna.borrow_mut().clone();
        let mut warna_akhir = self.warna.borrow_mut().clone();

        while warna_awal == warna_akhir {
            *self.warna.borrow_mut() = rand::random();
            warna_akhir = self.warna.borrow_mut().clone();
        }
    }

    pub fn set_warna(&self, warna: Warna) {
        *self.warna.borrow_mut() = warna;
    }
}

#[derive(Debug)]
pub struct Dempet {
    pub vektor: Vec<Posisi>
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
    /// use crate::permen_remuk::Dempet;
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
    /// use crate::permen_remuk::Dempet;
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

impl Deref for Dempet {
    type Target = Vec<Posisi>;

    fn deref(&self) -> &Self::Target {
        &self.vektor
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
        // write!(f, "\n")?;
        // write!(f, "   ")?;
        // for i in 0..self.isi[0].len() { write!(f, "{} ", i)?; }
        // write!(f, "\n")?;
        // for (y, baris) in self.isi.iter().enumerate() {
        //     write!(f, "{} ", y)?;
        for baris in self.isi.iter() {
            for permen in baris {
                write!(f, "{} ", permen)?;
            }
            write!(f, "\n")?;
        }
        // write!(f, "\n")?;
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
        self.isi[pos.0][pos.1].warna.borrow().clone()
    }

    /// Menukar warna dua permen yang bersebelahan
    ///
    /// ### Example
    /// ```
    /// use crate::permen_remuk::Papan;
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
        self.isi[pos1.0][pos1.1].set_warna(p2.warna.borrow().clone());
        self.isi[pos2.0][pos2.1].set_warna(p1.warna.borrow().clone());
    }

    /// Mengecek permen "dempet"
    /// (>=3 permen dengan warna yang sama saling besebelahan)
    ///
    /// cek ke atas, bawah, kiri, kanan dari posisi awal, misal:
    ///
    /// ```{bash}
    /// - - 0 - -    - 0 - - -   X 0 0 - -
    /// - - 0 - -    0 X 0 0 -   0 - - - -
    /// 0 0 X 0 0    - 0 - - -   0 - - - -
    /// - - 0 - -    - 0 - - -   - - - - -
    /// - - 0 - -    - - - - -   - - - - -
    /// ```
    pub fn cek_dempet(&self, pa: Posisi, tk: usize) -> Vec<Posisi> {
        // pa -> posisi awal | tk -> total kembar | tw -> target warna | pd -> posisi dempet
        // b_at -> batas atas | b_ba -> batas bawah | b_ki -> batas kiri | b_ka -> batas kanan
        let tw = self.get_warna(pa);
        let b_ki = if pa.1 < tk { 0 } else { pa.1 - (tk-1) };
        let b_ka = if pa.1 + tk > self.ukuran { self.ukuran } else { pa.1 + tk };

        let mut temp_x = Vec::<Posisi>::new();
        for (idx, x) in (b_ki..b_ka).enumerate() {
            if self.get_warna((pa.0, x)) == tw {
                temp_x.push((pa.0, x));
            } else {
                match idx {
                    0 => continue,
                    _ if idx == tk => break,
                    _ => if temp_x.len() < tk { temp_x = vec![] }
                }
            }
        }
        if temp_x.len() < tk { temp_x = vec![] }

        let b_at = if pa.0 < tk { 0 } else { pa.0 - (tk-1) };
        let b_ba = if pa.0 + tk > self.ukuran { self.ukuran } else { pa.0 + tk };

        let mut temp_y = Vec::<Posisi>::new();
        for (idx, y) in (b_at..b_ba).enumerate() {
            if self.get_warna((y, pa.1)) == tw {
                temp_y.push((y, pa.1));
            } else {
                match idx {
                    0 => continue,
                    _ if idx == tk => break,
                    _ => if temp_y.len() < tk { temp_y = vec![] }
                }
            }
        }
        if temp_y.len() < tk { temp_y = vec![] }

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

    /// Meremukan permen dengan mengganti permen dengan permen di atasnya
    pub fn remukan(&self, pos: Posisi) {
        let mut ps = pos.clone();
        while ps.0 != 0 {
            // ganti warna permen dengan warna permen di atas permen tersebut
            self.isi[ps.0][pos.1].set_warna(self.get_warna((ps.0-1, pos.1)));
            ps.0 -= 1;
        }
        self.isi[0][pos.1].acak_warna();
    }

    pub fn tukar_dan_remukan(&self, pos1: Posisi, pos2: Posisi) -> (usize, String) {
        self.tukar(pos1, pos2);
        let mut dempet = Dempet::new();

        dempet.tambah_vek(self.cek_dempet(pos1, 3))
            .tambah_vek(self.cek_dempet(pos2, 3))
            .urutkan();

        let mut total_remuk: usize = 0;
        let mut output = String::new();

        if dempet.len() == 0 {
            self.tukar(pos2, pos1);
            return (total_remuk, output);
        }

        output.push_str(&format!("{}\n{:?}\n", self, dempet));
        total_remuk += 1;

        for p in dempet.iter() {
            self.remukan(*p);
        }

        let mut tk: Vec<Posisi>;
        loop {
            tk = dempet.kolom_berubah();
            dempet.kosongkan();
            for mut pos in tk {
                loop {
                    dempet.tambah_vek(self.cek_dempet(pos, 3));
                    if pos.0 == 0 { break } else { pos.0 -= 1 };
                }
            }

            if dempet.len() == 0 {
                output.push_str(&format!("{}\n", self));
                break;
            }

            dempet.urutkan();

            output.push_str(&format!("{}\n{:?}\n", self, dempet));
            total_remuk += 1;

            for p in dempet.iter() {
                self.remukan(*p);
            }
        }

        (total_remuk, output)
    }

    /// Mengecek apakah permen dapat ditukar
    pub fn mungkin_ditukar(&self, pos: Posisi) -> bool {
        let mut mungkin = false;

        let tukar_dan_cek = |y, x| {
            let y0 = pos.0 as i16 + y;
            let x0 = pos.1 as i16 + x;
            let pos_target = (y0 as usize, x0 as usize);
            // println!("curr: {:?} -> target: {:?}", pos, pos_target);
            self.tukar(pos, pos_target);
            if self.cek_dempet(pos_target, 3).len() > 0 {
                // println!("{}", self.cek_dempet(pos_target, 3).len());
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
