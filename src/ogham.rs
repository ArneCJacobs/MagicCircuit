use std::iter;

use phf::{phf_map, Map};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ogham {
    B, // Beith
    L, // Luis
    F, // Fern
    S, // Sail
    N, // Nion

    H, // Uath
    D, // Dair
    T, // Tinne
    C, // Coll
    Q, // Ceirt

    M, // Muin
    G, // Gort
    NG, // Ngetal
    Z, // Straif
    R, // Ruis

    A, // Ailm
    O, // Onn
    U, // Ur
    E, // Eadhadh
    I, // Iodhadh

    EA, // Eabhadh
    OI, // Oir
    UI, // Uillean
    IA, // Ifin
    AE, // Eamhancholl
    P, // Peith

    Start, // Start
    End, // End
    Space, // Space
}

impl From<Ogham> for char {
    fn from(val: Ogham) -> Self {
        match val {
            Ogham::B => 'ᚁ',
            Ogham::L => 'ᚂ',
            Ogham::F => 'ᚃ',
            Ogham::S => 'ᚄ',
            Ogham::N => 'ᚅ',

            Ogham::H => 'ᚆ',
            Ogham::D => 'ᚇ',
            Ogham::T => 'ᚈ',
            Ogham::C => 'ᚉ',
            Ogham::Q => 'ᚊ',

            Ogham::M => 'ᚋ',
            Ogham::G => 'ᚌ',
            Ogham::NG => 'ᚍ',
            Ogham::Z => 'ᚎ',
            Ogham::R => 'ᚏ',

            Ogham::A => 'ᚐ',
            Ogham::O => 'ᚑ',
            Ogham::U => 'ᚒ',
            Ogham::E => 'ᚓ',
            Ogham::I => 'ᚔ',

            Ogham::EA => 'ᚕ',
            Ogham::OI => 'ᚖ',
            Ogham::UI => 'ᚗ',
            Ogham::IA => 'ᚘ',
            Ogham::AE => 'ᚚ',
            Ogham::P => 'ᚚ',

            Ogham::Start => '᚛',
            Ogham::End => '᚜',
            Ogham::Space => ' ',
        }
    }
}

const OGHAM_MAP: Map<char, Ogham> = phf_map!{
    'a' => Ogham::A,
    'b' => Ogham::B,
    'c' => Ogham::C,
    'd' => Ogham::D,
    'e' => Ogham::E,
    'f' => Ogham::F,
    'g' => Ogham::G,
    'h' => Ogham::H,
    'i' => Ogham::I,
    'j' => Ogham::H,
    'k' => Ogham::C,
    'l' => Ogham::L,
    'm' => Ogham::M,
    'n' => Ogham::N,
    'o' => Ogham::O,
    'p' => Ogham::P,
    'q' => Ogham::Q,
    'r' => Ogham::R,
    's' => Ogham::S,
    't' => Ogham::T,
    'u' => Ogham::U,
    'v' => Ogham::F,
    'w' => Ogham::EA,
    'x' => Ogham::OI,
    'y' => Ogham::IA,
    'z' => Ogham::Z,
    ' ' => Ogham::Space,
};

pub fn into_ogham(text: String) -> String {
    iter::once(Ogham::Start)
        .chain(
            text.to_lowercase()
                .chars()
                .map(|c| OGHAM_MAP.get(&c).unwrap_or(&Ogham::Space))
                .cloned()
        )
        .chain(iter::once(Ogham::End))
        .map(char::from)
        .collect()
}

