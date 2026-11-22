//! GGUFF Weight Editor v0.2 – Enhanced GUI app to record, edit, delete, and export weight entries.
//! Entries are saved as JSON in platform-appropriate data directory.
//! Run with `cargo run --release`.
//! Author: J. Roberto jimenez @hipotermiah

use chrono::{NaiveDate, Local};
use eframe::{egui, epi};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::Write;

/// One weight record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Entry {
    date: NaiveDate,
    weight_kg: f32,
}

/// The whole application state.
#[derive(Default, Serialize, Deserialize)]
struct AppState {
    entries: Vec<Entry>,
    /// UI helpers (not serialized)
    #[serde(skip)]
    new_date: String,
    #[serde(skip)]
    new_weight: String,
    #[serde(skip)]
    edit_index: Option<usize>,
    #[serde(skip)]
    error_msg: Option<String>,
    #[serde(skip)]
    success_msg: Option<String>,
    #[serde(skip)]
    show_stats: bool,
}

impl AppState {
    const FILE_NAME: &'static str = "weights.json";

    /// Get the proper file path for the current platform
    fn file_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("gguff-weight-editor")
    }

    /// Ensure the directory exists
    fn ensure_dir() {
        let path = Self::file_path();
        let _ = fs::create_dir_all(&path);
    }

    /// Full path to the data file
    fn full_path() -> PathBuf {
        Self::ensure_dir();
        Self::file_path().join(Self::FILE_NAME)
    }

    /// Load from disk, or start empty if the file is missing/corrupt.
    fn load() -> Self {
        let path = Self::full_path();
        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(state) => {
                    eprintln!("Loaded {} entries from {}", state.entries.len(), path.display());
                    state
                }
                Err(e) => {
                    eprintln!("Warning: Could not parse {}: {}", path.display(), e);
                    eprintln!("Starting with empty state.");
                    Default::default()
                }
            },
            Err(e) => {
                eprintln!("Info: No existing data file found at {}", path.display());
                eprintln!("Starting with empty state.");
                Default::default()
            }
        }
    }

    /// Persist current state to disk with backup.
    fn save(&self) {
        let path = Self::full_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            // Create backup before overwrite
            if path.exists() {
                let backup_path = path.with_extension("json.bak");
                let _ = fs::copy(&path, &backup_path);
            }
            
            if let Err(e) = fs::write(&path, json) {
                eprintln!("Error saving to {}: {}", path.display(), e);
            }
        }
    }

    /// Export entries to CSV
    fn export_csv(&self) -> Result<String, String> {
        if self.entries.is_empty() {
            return Err("No entries to export".into());
        }

        let mut csv_data = Vec::new();
        let mut writer = csv::WriterBuilder::new().from_writer(&mut csv_data);
        
        // Write header
        writer.write_record(&["Date", "Weight (kg)"])
            .map_err(|e| format!("CSV write error: {}", e))?;
        
        // Write entries
        for entry in &self.entries {
            writer.write_record(&[
                entry.date.format("%Y-%m-%d").to_string(),
                format!("{:.2}", entry.weight_kg),
            ]).map_err(|e| format!("CSV write error: {}", e))?;
        }
        
        writer.flush().map_err(|e| format!("CSV flush error: {}", e))?;
        
        Ok(String::from_utf8(csv_data).unwrap_or_else(|_| "UTF-8 conversion failed".into()))
    }

    /// Try to parse the temporary UI fields into an `Entry`.
    fn parse_new_entry(&self) -> Result<Entry, String> {
        let date = NaiveDate::parse_from_str(&self.new_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid date (YYYY-MM-DD): {}", e))?;
        
        let weight: f32 = self
            .new_weight
            .trim()
            .parse()
            .map_err(|e| format!("Invalid weight: {}", e))?;
        
        if weight <= 0.0 {
            return Err("Weight must be positive".into());
        }
        
        if weight > 500.0 {
            return Err("Weight seems unrealistic (>500 kg)".into());
        }
        
        if weight < 10.0 {
            return Err("Weight seems unrealistic (<10 kg)".into());
        }
        
        Ok(Entry { date, weight_kg: weight })
    }

    /// Check for duplicate dates
    fn has_duplicate_date(&self, date: NaiveDate, exclude_idx: Option<usize>) -> bool {
        self.entries.iter().enumerate().any(|(i, e)| {
            if let Some(exclude) = exclude_idx {
                i != exclude && e.date == date
            } else {
                e.date == date
            }
        })
    }

    /// Add a new entry (after validation).
    fn add_entry(&mut self) {
        match self.parse_new_entry() {
            Ok(entry) => {
                // Check for duplicates
                if self.has_duplicate_date(entry.date, None) {
                    self.error_msg = Some(format!(
                        "Warning: Entry already exists for {}. Adding anyway.",
                        entry.date
                    ));
                }
                
                self.entries.push(entry);
                self.entries.sort_by_key(|e| e.date);
                self.new_date.clear();
                self.new_weight.clear();
                self.error_msg = None;
                self.success_msg = Some("Entry added successfully!".to_string());
                self.save();
            }
            Err(err) => {
                self.error_msg = Some(err);
                self.success_msg = None;
            }
        }
    }

    /// Update an existing entry.
    fn apply_edit(&mut self) {
        if let Some(idx) = self.edit_index {
            match self.parse_new_entry() {
                Ok(entry) => {
                    // Check for duplicates (excluding current entry)
                    if self.has_duplicate_date(entry.date, Some(idx)) {
                        self.error_msg = Some(format!(
                            "Warning: Another entry exists for {}. Updating anyway.",
                            entry.date
                        ));
                    }
                    
                    self.entries[idx] = entry;
                    self.entries.sort_by_key(|e| e.date);
                    self.edit_index = None;
                    self.new_date.clear();
                    self.new_weight.clear();
                    self.error_msg = None;
                    self.success_msg = Some("Entry updated successfully!".to_string());
                    self.save();
                }
                Err(err) => {
                    self.error_msg = Some(err);
                    self.success_msg = None;
                }
            }
        }
    }

    /// Remove an entry at `idx`.
    fn delete_entry(&mut self, idx: usize) {
        if idx < self.entries.len() {
            self.entries.remove(idx);
            self.success_msg = Some("Entry deleted.".to_string());
            self.save();
        }
    }

    /// Fill the UI fields with the entry we want to edit.
    fn start_edit(&mut self, idx: usize) {
        if idx < self.entries.len() {
            let e = &self.entries[idx];
            self.new_date = e.date.format("%Y-%m-%d").to_string();
            self.new_weight = format!("{:.2}", e.weight_kg);
            self.edit_index = Some(idx);
            self.error_msg = None;
            self.success_msg = None;
        }
    }

    /// Set date to today
    fn set_today(&mut self) {
        self.new_date = Local::now().format("%Y-%m-%d").to_string();
    }

    /// Calculate statistics
    fn get_stats(&self) -> Option<Stats> {
        if self.entries.is_empty() {
            return None;
        }

        let first = self.entries.first().unwrap();
        let latest = self.entries.last().unwrap();
        let weights: Vec<f32> = self.entries.iter().map(|e| e.weight_kg).collect();
        
        let min = weights.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = weights.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let avg = weights.iter().sum::<f32>() / weights.len() as f32;
        let diff = latest.weight_kg - first.weight_kg;

        Some(Stats {
            first_date: first.date,
            first_weight: first.weight_kg,
            latest_date: latest.date,
            latest_weight: latest.weight_kg,
            min_weight: min,
            max_weight: max,
            avg_weight: avg,
            total_change: diff,
            entry_count: self.entries.len(),
        })
    }
}

#[derive(Debug)]
struct Stats {
    first_date: NaiveDate,
    first_weight: f32,
    latest_date: NaiveDate,
    latest_weight: f32,
    min_weight: f32,
    max_weight: f32,
    avg_weight: f32,
    total_change: f32,
    entry_count: usize,
}

/// Implements the `epi::App` trait required by `eframe`.
struct GguffApp {
    state: AppState,
    dark_mode: bool,
}

impl Default for GguffApp {
    fn default() -> Self {
        let mut state = AppState::load();
        // Pre-fill today's date for convenience
        state.set_today();
        Self {
            state,
            dark_mode: true,
        }
    }
}

impl epi::App for GguffApp {
    fn name(&self) -> &str {
        "GGUFF Weight Editor"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        // Apply theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🧭 GGUFF Weight Editor v0.2");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.dark_mode { "☀️ Light" } else { "🌙 Dark" }).clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ---- Success message ----
            if let Some(msg) = &self.state.success_msg {
                ui.colored_label(egui::Color32::GREEN, msg);
            }

            // ---- Input area (Add / Edit) ----
            ui.group(|ui| {
                ui.label(if self.state.edit_index.is_some() {
                    "✏️ Editing Entry"
                } else {
                    "➕ Add New Entry"
                });
                
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    ui.label("Date (YYYY-MM-DD):");
                    ui.text_edit_singleline(&mut self.state.new_date);
                    
                    if ui.button("📅 Today").clicked() {
                        self.state.set_today();
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Weight (kg):");
                    let weight_edit = ui.text_edit_singleline(&mut self.state.new_weight);
                    
                    if weight_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if self.state.edit_index.is_some() {
                            self.state.apply_edit();
                        } else {
                            self.state.add_entry();
                        }
                    }
                });
                
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    let btn_label = if self.state.edit_index.is_some() {
                        "✓ Apply Edit"
                    } else {
                        "➕ Add Entry"
                    };
                    
                    if ui.button(btn_label).clicked() {
                        if self.state.edit_index.is_some() {
                            self.state.apply_edit();
                        } else {
                            self.state.add_entry();
                        }
                    }
                    
                    if self.state.edit_index.is_some() {
                        if ui.button("❌ Cancel").clicked() {
                            self.state.edit_index = None;
                            self.state.new_date.clear();
                            self.state.new_weight.clear();
                            self.state.error_msg = None;
                            self.state.success_msg = None;
                            self.state.set_today();
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📊 Toggle Stats").clicked() {
                            self.state.show_stats = !self.state.show_stats;
                        }
                        
                        if ui.button("📤 Export CSV").clicked() {
                            match self.state.export_csv() {
                                Ok(csv_data) => {
                                    // Try to open save dialog
                                    if let Some(path) = rfd::FileDialog::new()
                                        .set_file_name("weights_export.csv")
                                        .save_file() {
                                        if let Ok(mut file) = fs::File::create(&path) {
                                            if file.write_all(csv_data.as_bytes()).is_ok() {
                                                self.state.success_msg = Some(format!(
                                                    "Exported to {}", path.display()
                                                ));
                                            } else {
                                                self.state.error_msg = Some("Failed to write CSV file".into());
                                            }
                                        } else {
                                            self.state.error_msg = Some("Failed to create CSV file".into());
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.state.error_msg = Some(e);
                                }
                            }
                        }
                    });
                });
            });

            // ---- Error message ----
            if let Some(err) = &self.state.error_msg {
                ui.colored_label(egui::Color32::RED, err);
            }

            ui.separator();

            // ---- Statistics Panel ----
            if self.state.show_stats {
                if let Some(stats) = self.state.get_stats() {
                    ui.group(|ui| {
                        ui.heading("📊 Statistics");
                        ui.add_space(4.0);
                        
                        egui::Grid::new("stats_grid")
                            .spacing([10.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Total Entries:");
                                ui.label(format!("{}", stats.entry_count));
                                ui.end_row();
                                
                                ui.label("First Entry:");
                                ui.label(format!("{} ({:.2} kg)", stats.first_date, stats.first_weight));
                                ui.end_row();
                                
                                ui.label("Latest Entry:");
                                ui.label(format!("{} ({:.2} kg)", stats.latest_date, stats.latest_weight));
                                ui.end_row();
                                
                                ui.label("Minimum Weight:");
                                ui.label(format!("{:.2} kg", stats.min_weight));
                                ui.end_row();
                                
                                ui.label("Maximum Weight:");
                                ui.label(format!("{:.2} kg", stats.max_weight));
                                ui.end_row();
                                
                                ui.label("Average Weight:");
                                ui.label(format!("{:.2} kg", stats.avg_weight));
                                ui.end_row();
                                
                                ui.label("Total Change:");
                                ui.colored_label(
                                    if stats.total_change < 0.0 { 
                                        egui::Color32::GREEN 
                                    } else if stats.total_change > 0.0 { 
                                        egui::Color32::RED 
                                    } else { 
                                        egui::Color32::GRAY 
                                    },
                                    format!("{:+.2} kg", stats.total_change),
                                );
                                ui.end_row();
                            });
                    });
                    ui.add_space(8.0);
                }
            }

            // ---- Table of entries ----
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("📋 Entries ({})", self.state.entries.len()));
                });
                
                ui.add_space(4.0);
                
                if self.state.entries.is_empty() {
                    ui.label("No entries yet. Add your first weight above!");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("entries_grid")
                            .striped(true)
                            .spacing([10.0, 4.0])
                            .show(ui, |ui| {
                                ui.heading("Date");
                                ui.heading("Weight (kg)");
                                ui.heading("Actions");
                                ui.end_row();

                                for (i, entry) in self.state.entries.iter().enumerate() {
                                    ui.label(entry.date.format("%Y-%m-%d").to_string());
                                    ui.label(format!("{:.2}", entry.weight_kg));
                                    ui.horizontal(|ui| {
                                        if ui.small_button("✏️ Edit").clicked() {
                                            self.state.start_edit(i);
                                        }
                                        if ui.small_button("🗑 Delete").clicked() {
                                            self.state.delete_entry(i);
                                        }
                                    });
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        });
    }
}

fn main() {
    // Enable native DPI scaling on Windows/macOS
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "GGUFF Weight Editor",
        native_options,
        Box::new(|_cc| Box::new(GguffApp::default())),
    );
}
