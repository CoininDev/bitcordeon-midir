use midir::{MidiOutput, MidiOutputConnection, os::unix::VirtualOutput};

use crate::midi::{clear_notes, orchestrate};
use crate::music::{self, State};
pub struct MyEguiApp {
    pub conn_out: Option<MidiOutputConnection>,
    pub state: State,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let midi_out = MidiOutput::new("BitCordeon").unwrap();
        let conn_out = Some(midi_out.create_virtual("BitCordeon Out").unwrap());
        Self {
            conn_out: conn_out,
            state: State::default(),
        }
    }

    fn step_music(&mut self, ctx: &egui::Context) {
        let b1 = ctx.input(|i| i.key_down(egui::Key::Num1)) as u8;
        let b2 = ctx.input(|i| i.key_down(egui::Key::Num2)) as u8;
        let b3 = ctx.input(|i| i.key_down(egui::Key::Num3)) as u8;
        let sharp = ctx.input(|i| i.key_down(egui::Key::Num4)) as u8;
        self.state.minor = ctx.input(|i| i.key_down(egui::Key::Escape));
        self.state.sept = ctx.input(|i| i.key_down(egui::Key::Q));
        self.state.playing = ctx.input(|i| i.key_down(egui::Key::Space));

        let bitmap = b1 << 2 | b2 << 1 | b3;

        if ctx.input(|i| i.key_down(egui::Key::Semicolon)) {
            self.state.single = !self.state.single;
        }

        if bitmap != 0 {
            let chromatic = music::simple_to_chromatic(bitmap - 1) + sharp; // 0..11
            let root_midi = 60u8 + chromatic; // 60 é C4; chromatic 0 -> C4, 11 -> B4
            self.state.current_note = Some(root_midi);
        } else {
            self.state.current_note = None;
        }
    }

    fn play_note(&mut self) {
        // Se não tem conexão MIDI, apenas manutenção de estado local
        if self.conn_out.is_none() {
            if !self.state.playing {
                self.state.current_midi_notes.clear();
                self.state.last_note = None;
                // zera os last modes também
                self.state.last_minor = false;
                self.state.last_sept = false;
            }
            return;
        }

        // Pega a conexão mutável só uma vez
        if let Some(conn) = self.conn_out.as_mut() {
            // Se não estiver tocando, garante que tudo pare
            if !self.state.playing {
                self.state = clear_notes(&self.state, conn);
                self.state.last_note = None;
                self.state.last_minor = false;
                self.state.last_sept = false;
                return;
            }

            // Aqui: playing == true
            match (self.state.last_note, self.state.current_note) {
                // nota mudou -> para antigas e toca novas
                (Some(last), Some(current)) if last != current => {
                    self.state = clear_notes(&self.state, conn);
                    self.state = orchestrate(&self.state, conn, current);
                    self.state.last_note = Some(current);
                    self.state.last_minor = self.state.minor;
                    self.state.last_sept = self.state.sept;
                    self.state.last_single = self.state.single;
                }

                // mesma nota, mas parâmetros (minor/sept) mudaram -> retrigger
                (Some(last), Some(current)) if last == current => {
                    if self.state.minor != self.state.last_minor
                        || self.state.sept != self.state.last_sept
                        || self.state.last_single != self.state.single
                    {
                        // re-aplica o acorde com os novos flags
                        self.state = clear_notes(&self.state, conn);
                        self.state = orchestrate(&self.state, conn, current);
                        self.state.last_minor = self.state.minor;
                        self.state.last_sept = self.state.sept;
                        self.state.last_single = self.state.single;
                        // last_note já é current; mantemos last_note
                    } else {
                        // nada a fazer, mesma nota e mesmos parâmetros
                    }
                }

                (None, Some(current)) => {
                    self.state = orchestrate(&self.state, conn, current);
                    self.state.last_note = Some(current);
                    self.state.last_minor = self.state.minor;
                    self.state.last_sept = self.state.sept;
                    self.state.last_single = self.state.single;
                }

                // tinha nota, agora não há current -> limpa
                (Some(_), None) => {
                    self.state = clear_notes(&self.state, conn);
                    self.state.last_note = None;
                    self.state.last_minor = false;
                    self.state.last_sept = false;
                    self.state.last_single = false;
                }

                // nada a fazer para (None, None)
                (None, None) => {}
                (Some(_), Some(_)) => {}
            }
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.step_music(ctx);
        self.play_note();

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(note) = self.state.current_note {
                // note é um valor MIDI (0..127)
                let name_idx = (note % 12) as usize;
                let name = music::GRAPHICAL_NOTES[name_idx];
                // C4 = 60 -> octave = 4  (fórmula: octave = (midi / 12) - 1)
                let octave = (note as i32 / 12) - 1;

                let minor = if self.state.minor { "m" } else { "" };
                let sept = if self.state.sept { "7" } else { "" };
                let single = if self.state.single {
                    "(single)"
                } else {
                    "(chord)"
                };
                let playing = if self.state.playing {
                    "-- playing --"
                } else {
                    ""
                };

                ui.horizontal(|ui| {
                    ui.label(format!("{}{}{}. oct:{}", name, minor, sept, octave));
                    ui.label(format!("MIDI:{}", note));
                });
                ui.label(single);
                ui.label(playing);
            }
        });
    }
}
