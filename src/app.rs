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
        if let Some(storage) = cc.storage {
            let mut state: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            state.units = serde_json::from_str(include_str!("../units.json")).unwrap();
            return state;
        }

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
            ui.heading("verfuegbare Ausruestung");

            for unit in units {
                let r = ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        if let Some(icon) = &unit.icon {
                            ui.label(icon);
                        }
                        ui.label(&unit.name);
                        ui.hyperlink_to("wiki", &unit.url);
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{}",
                            unit.kaufpreis.to_formatted_string(&Locale::de)
                        ));
                        if ui.button("+1").clicked() {
                            *units_bought.entry(unit.clone()).or_default() += 1;
                        }
                        if ui.button("+10").clicked() {
                            *units_bought.entry(unit.clone()).or_default() += 10;
                        }
                    });
                }).response;

                if unit.beschreibung != "" {
                    r.on_hover_text(&unit.beschreibung);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hallo! Hier koennen Sie das Sondervermoegen der Bundeswehr ausgeben!");

            ui.heading(format!(
                "Verfuegbarer Betrag: {}",
                remaining_credits.to_formatted_string(&Locale::de)
            ));

            let mut cost = 0;

            for (unit, num) in units_bought {
                if *num == 0 {continue;}
                let price = *num as i64 * unit.total_cost();

                cost += price;


                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} x {} {}",
                        num,
                        &unit.name,
                        price.to_formatted_string(&Locale::de)
                    ));
    
                    if ui.button("🗑").clicked() {
                        *num = 0;
                    }
                });
            }

            *remaining_credits = *credits - cost;

            egui::CollapsingHeader::new("Info")
                .default_open(false)
                .show(ui, |ui| {
                    ui.hyperlink_to("Github", "https://github.com/woelper/sondervermoegen");
                    ui.hyperlink_to(
                        "Konfiguration der Einheiten:",
                        " https://github.com/woelper/sondervermoegen/blob/master/units.json",
                    );
                });
        });
    }
}
