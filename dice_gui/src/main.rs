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
}

impl Default for DiceApp {
    fn default() -> Self {
        Self {
            notation: "2d6+1".to_owned(),
            last_roll: None,
            error_message: String::new(),
            history: Vec::new(),
        }
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

            if ui.button("Roll").clicked() {
                // Borrowing example: pass &self.notation without moving ownership.
                match make_roll(&self.notation) {
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
