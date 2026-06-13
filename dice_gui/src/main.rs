use dice_core::{make_roll, make_roll_owned, Roll};
use eframe::egui;

fn main() -> eframe::Result<()> {
    // CLI fallback using the same core library:
    // cargo run -p dice_gui -- 2d6+1
    if let Some(notation) = std::env::args().nth(1) {
        match make_roll_owned(notation) {
            Ok(roll) => {
                println!("Notation: {}", roll.notation);
                println!("Results: {:?}", roll.results);
                println!("Total: {}", roll.total);
            }
            Err(err) => {
                eprintln!("Error: {err}");
            }
        }
        return Ok(());
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust D&D Dice Roller",
        options,
        Box::new(|_cc| Box::<DiceApp>::default()),
    )
}

struct DiceApp {
    notation: String,
    last_roll: Option<Roll>,
    error_message: String,
    history: Vec<String>,
    favorites: Vec<String>,
}

impl Default for DiceApp {
    fn default() -> Self {
        Self {
            notation: "2d6+1".to_owned(),
            last_roll: None,
            error_message: String::new(),
            history: Vec::new(),
            favorites: Vec::new(),
        }
    }
}

impl DiceApp {
    fn run_roll_notation(&mut self, notation: &str) {
        // Borrowing example: pass notation without moving ownership.
        match make_roll(notation) {
            Ok(roll) => {
                self.error_message.clear();
                self.history
                    .push(format!("{} => {:?} = {}", roll.notation, roll.results, roll.total));
                self.last_roll = Some(roll);
            }
            Err(err) => {
                self.last_roll = None;
                self.error_message = err.to_string();
            }
        }
    }

    fn add_favorite(&mut self) {
        let notation = self.notation.trim();
        if notation.is_empty() {
            self.error_message = "cannot favorite an empty notation".to_string();
            return;
        }

        if self.favorites.iter().any(|f| f == notation) {
            self.error_message = "favorite already exists".to_string();
            return;
        }

        self.favorites.push(notation.to_owned());
        self.error_message.clear();
    }
}

impl eframe::App for DiceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("D&D Dice Roller");
            ui.label("Enter notation such as 2d6+1, d20, or 4d8-2");

            ui.horizontal(|ui| {
                ui.label("Notation:");
                ui.text_edit_singleline(&mut self.notation);
            });

            ui.horizontal(|ui| {
                if ui.button("Roll").clicked() {
                    let notation = self.notation.clone();
                    self.run_roll_notation(&notation);
                }

                if ui.button("Favorite This Roll").clicked() {
                    self.add_favorite();
                }
            });

            if !self.favorites.is_empty() {
                ui.separator();
                ui.heading("Favorite Rolls");

                let mut remove_index: Option<usize> = None;

                // Loop over favorites so users can reroll with one click.
                for idx in 0..self.favorites.len() {
                    let favorite = self.favorites[idx].clone();
                    ui.horizontal(|ui| {
                        ui.label(&favorite);

                        if ui.button("Roll Favorite").clicked() {
                            self.notation = favorite.clone();
                            self.run_roll_notation(&favorite);
                        }

                        if ui.button("Use").clicked() {
                            self.notation = favorite.clone();
                            self.error_message.clear();
                        }

                        if ui.button("Remove").clicked() {
                            remove_index = Some(idx);
                        }
                    });
                }

                if let Some(idx) = remove_index {
                    self.favorites.remove(idx);
                }
            }

            ui.separator();

            if !self.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", self.error_message));
            }

            if let Some(roll) = &self.last_roll {
                ui.label(format!("Results (Vec): {:?}", roll.results));
                ui.label(format!("Total: {}", roll.total));
            }

            ui.separator();
            ui.label("Recent rolls:");
            // Loop requirement: iterate over roll history for display.
            for entry in self.history.iter().rev().take(5) {
                ui.label(entry);
            }
        });
    }
}
