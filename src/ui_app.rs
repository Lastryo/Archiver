use eframe::{egui::CentralPanel, egui::CtxRef, epi::App, epi::Frame};

mod zip_archiver;

pub struct Archiver {
    directory: String,
    destination: String,
    extension: Extension,
    group: bool,
}

impl Archiver {
    pub fn new(directory: String, destination: String, ext: Extension, group: bool) -> Archiver {
        Archiver {
            directory: directory,
            destination: destination,
            extension: ext,
            group: group,
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
            
            let group_match = match self.group {
                true => true,
                false => {
                    ui.horizontal(|ui| {
                        ui.label("destination: ");
                        ui.text_edit_singleline(&mut self.destination);
                    });

                    false
                }
            };

            ui.horizontal(|ui| {
                ui.label("directory: ");
                ui.text_edit_singleline(&mut self.directory);
            });

            if ui.button("Create").clicked() {
                match group_match {
                    true => {
                        zip_archiver::create_archives(
                            &self.directory,
                            &get_extention(&self.extension),
                        );
                    }
                    false => {
                        zip_archiver::create_archive(
                            &self.directory,
                            &self.destination,
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
