use crate::music::{CHORD_HANDS, State, get_scale_from_midi};
use midir::MidiOutputConnection;

pub fn orchestrate(state: &State, conn: &mut MidiOutputConnection, current: u8) -> State {
    let mut s = state.clone();
    if let None = state.current_note {
        return s;
    }
    if state.single {
        s = play_single_note(&s, conn, current);
        s
    } else {
        let hand = if !s.sept {
            CHORD_HANDS[1]
        } else {
            CHORD_HANDS[2]
        };

        let scale = get_scale_from_midi(s.current_note.unwrap(), s.minor);
        for &finger in hand.iter() {
            let note_midi = scale[finger as usize];
            s = play_single_note(&s, conn, note_midi);
        }

        s
    }
}

pub fn play_single_note(state: &State, conn: &mut MidiOutputConnection, current: u8) -> State {
    let note = current;
    conn.send(&[0x90, note, 100])
        .expect("impossível tocar nota");
    let mut s = state.clone();
    s.current_midi_notes.push(note);
    println!("tocou nota {}", note);
    s
}

pub fn clear_notes(state: &State, conn: &mut MidiOutputConnection) -> State {
    let mut s = state.clone();
    let mut t = s.clone();
    for &note in &s.current_midi_notes {
        t = stop_single_note(&s, conn, note);
    }
    t.current_midi_notes.clear();
    s = t;
    return s;
}

pub fn stop_single_note(state: &State, conn: &mut MidiOutputConnection, last: u8) -> State {
    let note = last;
    conn.send(&[0x80, note, 0]).expect("impossível parar nota");
    let mut s = state.clone();
    s.current_midi_notes.retain(|&v| v != note);
    println!("parou nota {}", note);
    s
}
