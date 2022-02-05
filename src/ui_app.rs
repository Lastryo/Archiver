use std::path::PathBuf;

use eframe::{egui::CentralPanel, egui::CtxRef, epi::App, epi::Frame};
use im_native_dialog::ImNativeFileDialog;

mod zip_archiver;

#[derive()]
pub struct Archiver {
    directory_path: PathBuf,
    destination_path: PathBuf,
    directory_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
    destination_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
    extension: Extension,
    group: bool,
}

impl Archiver {
    pub fn new(ext: Extension, group: bool) -> Archiver {
        Archiver {
            extension: ext,
            group: group,
            destination_path: Default::default(),
            directory_path: Default::default(),
            directory_path_dialog: Default::default(),
            destination_path_dialog: Default::default(),
        }
    }
}

impl App for Archiver {
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        if let Some(result) = self.directory_path_dialog.check() {
            match result {
                Ok(Some(path)) => self.directory_path = path,
                Ok(None) => {}
                Err(err) => {
                    eprintln!("error selecting xplane_path: {}", err)
                }
            }
        }

        if let Some(result) = self.destination_path_dialog.check() {
            match result {
                Ok(Some(path)) => self.destination_path = path,
                Ok(None) => {}
                Err(err) => {
                    eprintln!("error selecting xplane_path: {}", err)
                }
            }
        }

        let directory_path = &self.directory_path.to_string_lossy().to_string();
        let destination_path = &self.destination_path.to_string_lossy().to_string();

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Create archive");
            ui.horizontal(|ui| {
                ui.label("convert type: ");
                ui.radio_value(&mut self.extension, Extension::Zip, "Zip");
                ui.radio_value(&mut self.extension, Extension::Cbz, "Cbz");
            });

            ui.horizontal(|ui| {
                ui.label("archive type: ");
                ui.radio_value(&mut self.group, false, "One");
                ui.radio_value(&mut self.group, true, "Group");
            });

            ui.horizontal(|ui| {
                ui.label("directory: ");
                ui.horizontal(|ui| {
                    ui.label(directory_path);
                    // ui.text_edit_singleline(&mut self.directory);
                    if ui.button("Browse").clicked() {
                        let location = self
                            .directory_path
                            .parent()
                            .map(|location| location.to_path_buf());

                        let repaint_signal = _frame.request_repaint();
                        self.directory_path_dialog
                            .with_callback(move |_| repaint_signal)
                            .open_single_dir(location)
                            .expect("Unable to open file_path dialog");
                    }
                });
            });

            let group_match = match self.group {
                true => true,
                false => {
                    ui.horizontal(|ui| {
                        ui.label("destination: ");
                        ui.horizontal(|ui| {
                            ui.label(destination_path);
                            if ui.button("Browse").clicked() {
                                let location = self
                                    .destination_path
                                    .parent()
                                    .map(|location| location.to_path_buf());

                                let repaint_signal = _frame.request_repaint();
                                self.destination_path_dialog
                                    .with_callback(move |_| repaint_signal)
                                    .open_single_dir(location)
                                    .expect("Unable to open file_path dialog");
                            }
                        });
                    });

                    false
                }
            };

            if ui.button("Create").clicked() {
                match group_match {
                    true => {
                        zip_archiver::create_archives(
                            &directory_path,
                            &get_extention(&self.extension),
                        );
                    }
                    false => {
                        zip_archiver::create_archive(
                            &directory_path,
                            &destination_path,
                            &get_extention(&self.extension),
                        );
                    }
                }
            }
        });
    }

    fn name(&self) -> &str {
        "Archiver"
    }
}

fn get_extention(ext: &Extension) -> String {
    match ext {
        Extension::Zip => ".zip".to_string(),
        Extension::Cbz => ".cbz".to_string(),
    }
}

#[derive(PartialEq)]
pub enum Extension {
    Zip,
    Cbz,
}
