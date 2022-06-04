mod resources;
use num_format::{Locale, ToFormattedString};
use resources::*;
use std::collections::HashMap;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    credits: i64,
    remaining_credits: i64,
    units: Vec<Unit>,
    units_bought: HashMap<Unit, usize>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            credits: 100_000_000_000,
            remaining_credits: 100_000_000_000,
            units: serde_json::from_str(include_str!("../units.json")).unwrap(),
            units_bought: HashMap::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {

        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            credits,
            remaining_credits,
            units,
            units_bought,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Einheiten zum Kauf");

            for unit in units {
                ui.horizontal(|ui| {
                    ui.label(&unit.name);
                    ui.label(format!("{}", unit.kaufpreis.to_formatted_string(&Locale::de)));
                    if ui.button("+1").clicked() {
                        *units_bought.entry(unit.clone()).or_default() += 1;
                    }
                    if ui.button("+10").clicked() {
                        *units_bought.entry(unit.clone()).or_default() += 10;
                    }
                });
            }


 
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading(format!(
                "Restvermoegen {}",
                remaining_credits.to_formatted_string(&Locale::de)
            ));

            let mut cost = 0;

            for (unit, num) in units_bought {
                let price = *num as i64 * unit.total_cost();

                cost += price;

                ui.label(format!(
                    "{} x {} {}",
                    num,
                    &unit.name,
                    price.to_formatted_string(&Locale::de)
                ));
            }

            *remaining_credits = *credits - cost;

 

            
        });

    }
}
