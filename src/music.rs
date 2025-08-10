#[derive(Default, PartialEq, Eq, Clone)]
pub struct State {
    pub note_index: Option<u8>,
    pub minor: bool,
    pub single: bool,
    pub sept: bool,
    pub current_note: Option<u8>,
    pub current_midi_notes: Vec<u8>,
    pub playing: bool,

    pub last_note: Option<u8>,
    pub last_minor: bool,
    pub last_sept: bool,
    pub last_single: bool,
}

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
    0,  // ??? (invalid)
];

pub const GRAPHICAL_NOTES: [&str; 13] = [
    "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B", "?????",
];

pub const MAJOR_STEPS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const MINOR_STEPS: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

pub const CHORD_HANDS: [&[u8]; 3] = [&[0], &[0, 2, 4], &[0, 2, 4, 6]];

pub fn get_scale_from_midi(root_midi: u8, minor: bool) -> Vec<u8> {
    let steps = if minor { &MINOR_STEPS } else { &MAJOR_STEPS };
    let mut scale = Vec::with_capacity(steps.len());

    for &step in steps {
        // usa u16 temporário para evitar overflow ao somar e depois checa limite 0..=127
        let val = root_midi as u16 + step as u16;
        if val > 127 {
            // evita overflow; saturamos em 127 (ou você pode descartar / usar outra policy)
            scale.push(127u8);
        } else {
            scale.push(val as u8);
        }
    }

    scale
}

pub fn simple_to_chromatic(simple: u8) -> u8 {
    // converte simple 0..6 para o passo cromático relativo (0..11)
    MAJOR_STEPS[simple as usize]
}
