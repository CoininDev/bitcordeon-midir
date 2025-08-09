// #[derive(Default, PartialEq, Eq)]
// pub struct State {
//     pub note_index: Option<u8>,
//     //pub sharp: bool,
//     pub minor: bool,
//     pub unique: bool,
// }

pub const NOTES: [u8; 13] = [
    60, // C
    61, // C#/Db
    62, // D
    63, // D#/Eb
    64, // E
    65, // F
    66, // F#/Gb
    67, // G
    68, // G#/Ab
    69, // A
    70, // A#/Bb
    71, // B
    0,  // ???
];

pub const GRAPHICAL_NOTES: [&str; 13] = [
    "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B", "?????",
];

pub const MAJOR_STEPS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const MINOR_STEPS: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

use lazy_static::lazy_static;

lazy_static! {
    static ref CHORD_HANDS: [Vec<u8>; 3] = [vec![0], vec![0, 2, 4], vec![0, 2, 4, 6],];
}

pub fn get_scale(root: u8, minor: bool) -> Vec<u8> {
    let steps = if minor { &MINOR_STEPS } else { &MAJOR_STEPS };
    let mut scale = Vec::new();

    for &step in steps {
        let index = (root + step) % 14;
        scale.push(NOTES[index as usize]);
    }

    scale
}

pub fn simple_to_chromatic(simple: u8) -> u8 {
    // convert simple 0-7 to chromatic 0-12 notes
    MAJOR_STEPS[simple as usize]
}
