use midir::{MidiOutput, MidiOutputConnection, os::unix::VirtualOutput};

use crate::music::{self, NOTES};

pub struct MyEguiApp {
    pub single: bool,
    pub minor: bool,
    pub conn_out: Option<MidiOutputConnection>,
    pub last_note: Option<u8>,
    pub current_note: Option<u8>,
    pub current_midi_notes: Vec<u8>,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let midi_out = MidiOutput::new("BitCordeon").unwrap();
        let conn_out = Some(midi_out.create_virtual("BitCordeon Out").unwrap());
        Self {
            single: false,
            minor: false,
            conn_out: conn_out,
            current_note: None,
            last_note: None,
            current_midi_notes: vec![],
        }
    }

    fn step_music(&mut self, ctx: &egui::Context) {
        let b1 = ctx.input(|i| i.key_down(egui::Key::Num1)) as u8;
        let b2 = ctx.input(|i| i.key_down(egui::Key::Num2)) as u8;
        let b3 = ctx.input(|i| i.key_down(egui::Key::Num3)) as u8;
        let sharp = ctx.input(|i| i.key_down(egui::Key::Num4)) as u8;
        self.minor = ctx.input(|i| i.key_down(egui::Key::Escape));

        let bitmap = b1 << 2 | b2 << 1 | b3;

        if ctx.input(|i| i.key_down(egui::Key::Semicolon)) {
            self.single = !self.single;
        }

        if bitmap != 0 {
            self.current_note = Some((music::simple_to_chromatic(bitmap - 1) + sharp) % 14);
        } else {
            self.current_note = None;
        }
        dbg!(self.current_note);
    }

    fn interpretate_note(&self, conn: &mut MidiOutputConnection, current: u8, last: u8) {
        if self.single {
            self.play_single_note(conn, current);
        } else {
        }
    }

    fn play_single_note(&self, conn: &mut MidiOutputConnection, current: u8) {
        conn.send(&[0x90, NOTES[current as usize], 100])
            .expect("impossível tocar nota");
    }
    fn clear_notes(&mut self, conn: &mut MidiOutputConnection) {
        for &note in &self.current_midi_notes {
            self.stop_single_note(conn, note);
        }
        self.current_midi_notes.clear();
    }
    fn stop_single_note(&self, conn: &mut MidiOutputConnection, last: u8) {
        conn.send(&[0x80, NOTES[last as usize], 0])
            .expect("impossível parar nota");
    }

    fn play_note(&mut self) {
        if let Some(ref mut conn) = self.conn_out {
            match (self.last_note, self.current_note) {
                (Some(last), Some(current)) if last != current => {
                    self.clear_notes(conn);
                    // Liga a nota nova
                    self.last_note = Some(current);
                }
                (None, Some(current)) => {
                    // Liga a nova nota
                    conn.send(&[0x90, NOTES[current as usize], 100])
                        .expect("impossível tocar nota");
                    self.last_note = Some(current);
                }
                (Some(last), None) => {
                    // Desliga a nota antiga
                    conn.send(&[0x80, NOTES[last as usize], 0])
                        .expect("impossível parar nota");
                    self.last_note = None;
                }
                (None, None) => {
                    // Nenhuma nota ligada, nada a fazer
                }
                (Some(last), Some(current)) if last == current => {
                    // Mesma nota, nada a fazer
                }
                (Some(_), Some(_)) => {
                    // erro
                }
            }
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.step_music(ctx);
        self.play_note();
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(note) = self.current_note {
                ui.label(music::GRAPHICAL_NOTES[note as usize]);
                ui.label(music::NOTES[note as usize].to_string());
            }
        });
    }
}
