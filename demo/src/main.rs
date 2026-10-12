#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use std::cell::RefCell;
use std::sync::Arc;
use eframe::egui;
use eframe::egui::{Color32, TextStyle};
use egui::{Id, Stroke};
use egui_extras::{Size, StripBuilder};
use egui_marquee::Interactive;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    first_interactive_color: RefCell<egui::Color32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            first_interactive_color: RefCell::new(Color32::RED),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.stack().bg_color();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        let body_text_size = TextStyle::Body.resolve(ui.style()).size;

        egui_marquee::Marquee::new(Id::new("demo"))
            .stroke(Stroke::new(1.0, Color32::RED))
            .show(ui, |ui| {
                StripBuilder::new(ui)
                    .size(Size::exact(50.0))
                    .size(Size::remainder())
                    .size(Size::relative(0.5).at_least(60.0))
                    .size(Size::exact(body_text_size))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();

                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                faded_color(Color32::BLUE),
                            );

                            let label = egui::Label::new("goo goo").wrap().interactive(
                                "test_interactivity_1",
                                |label, _, _| {
                                    *label = egui::Label::new("gaga");
                                },
                                |_,_| {}
                            );

                            ui.add_sized(rect.size(), label);
                        });
                        strip.strip(|builder| {
                            builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let rect = ui.available_rect_before_wrap();

                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        0.0,
                                        faded_color(Color32::RED),
                                    );
                                    let label = egui::Label::new("foo").wrap().interactive(
                                        "test_interactivity_2",
                                        |label, _, _| {
                                            *label = egui::Label::new("bar");
                                        },
                                        |_,_| {}
                                    );

                                    ui.add_sized(rect.size(), label);
                                });
                                strip.strip(|builder| {
                                    builder.sizes(Size::remainder(), 3).vertical(|mut strip| {
                                        strip.empty();
                                        strip.cell(|ui| {
                                            ui.painter().rect_filled(
                                                ui.available_rect_before_wrap(),
                                                0.0,
                                                faded_color(Color32::YELLOW),
                                            );
                                            ui.label("width: 50%\nheight: 1/3 of the red region");
                                        });
                                        strip.empty();
                                    });
                                });
                            });
                        });
                        strip.strip(|builder| {
                            builder
                                .size(Size::remainder())
                                .size(Size::exact(120.0))
                                .size(Size::remainder())
                                .size(Size::exact(70.0))
                                .horizontal(|mut strip| {
                                    strip.empty();
                                    strip.strip(|builder| {
                                        builder
                                            .size(Size::remainder())
                                            .size(Size::exact(60.0))
                                            .size(Size::remainder())
                                            .vertical(|mut strip| {
                                                strip.empty();
                                                strip.cell(|ui| {
                                                    let rect = ui.available_rect_before_wrap();


                                                    let label = egui::Label::new(":(").wrap().interactive(
                                                        ":|",
                                                        |label, _, _| {
                                                            *label = egui::Label::new(":)");
                                                            *self.first_interactive_color.borrow_mut() = Color32::GREEN;
                                                        },
                                                        |_,_| {
                                                            *self.first_interactive_color.borrow_mut() = Color32::RED;
                                                        }
                                                    );

                                                    {
                                                        let col = *self.first_interactive_color.borrow();

                                                        println!("COL: {col:?}");

                                                        ui.painter().rect_filled(
                                                            ui.available_rect_before_wrap(),
                                                            0.0,
                                                            faded_color(col),
                                                        );
                                                    }

                                                    ui.add_sized(rect.size(), label);
                                                });
                                            });
                                    });
                                    strip.empty();
                                    strip.cell(|ui| {
                                        ui.painter().rect_filled(
                                            ui.available_rect_before_wrap(),
                                            0.0,
                                            faded_color(Color32::GREEN),
                                        );
                                        ui.label("width: 70px\n\nheight: 50%, but at least 60px.");
                                    });
                                });
                        });
                        // strip.cell(|ui| {
                        //     ui.vertical_centered(|ui| {
                        //         ui.add(crate::egui_github_link_file!());
                        //     });
                        // });
                    });
            });
    }
}
