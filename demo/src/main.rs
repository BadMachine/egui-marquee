#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use eframe::egui::{Color32, TextStyle};
use egui::Shape::Vec;
use egui::{Id, Stroke};
// use egui_extras::SvgGlyph;
use egui_extras::{Size, StripBuilder};
use egui_marquee::Interactive;
use std::cell::RefCell;
// Uncomment for 0.36.3
// /// A private use character, rendered as Ferris.
// const FERRIS: char = '\u{E000}';
//
// /// A regular character whose glyph we override.
// const HEART: char = '♥';

const FERRIS: &[u8] = include_bytes!("../assets/ferris.svg");
const HEART: &[u8] = include_bytes!("../assets/heart.svg");

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
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

// fn install_svg_glyphs(ctx: &egui::Context) -> Result<(), String> {
//     // The heart is white, so it is tinted with the text color.
//     // It is a bit smaller than the font size, and lifted slightly above the baseline:
//     let heart = SvgGlyph::from_bytes(include_bytes!("../assets/heart.svg"))?
//         .with_height(0.7)
//         .with_baseline_offset(0.05);
//     ctx.add_glyph_rasterizer(heart.into_rasterizer(HEART.to_string()));
//
//     // Ferris is colorful, so the colors are kept:
//     let ferris = SvgGlyph::from_bytes(include_bytes!("../assets/ferris.svg"))?;
//     ctx.add_glyph_rasterizer(ferris.into_rasterizer(FERRIS.to_string()));

//     Ok(())
// }

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

        egui_marquee::Marquee::new("demo")
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

                            let label = egui::Label::new("goo goo")
                                .wrap()
                                .interactive("test_interactivity_1")
                                .inside(|label, _| {
                                    *label = egui::Label::new("gaga");
                                });

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
                                    let label = egui::Label::new("foo")
                                        .wrap()
                                        .interactive("test_interactivity_2")
                                        .inside(|label, _| {
                                            *label = egui::Label::new("bar");
                                        });

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

                                                    let label = egui::Label::new(":(")
                                                        .wrap()
                                                        .interactive(":|")
                                                        .inside(|label, _| {
                                                            *label = egui::Label::new(":)");
                                                            *self
                                                                .first_interactive_color
                                                                .borrow_mut() = Color32::GREEN;
                                                        })
                                                        .outside(|_| {
                                                            *self
                                                                .first_interactive_color
                                                                .borrow_mut() = Color32::RED;
                                                        })
                                                        .end(|_, _| {
                                                            println!("HOORA!");
                                                        });

                                                    {
                                                        let col =
                                                            *self.first_interactive_color.borrow();

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
                                    strip.cell(|ui| {
                                        ui.painter().rect_filled(
                                            ui.available_rect_before_wrap(),
                                            0.0,
                                            faded_color(Color32::GREEN),
                                        );

                                        let image =
                                            egui::Image::from_bytes("bytes://ferris.svg", FERRIS)
                                                .interactive("luv_rust")
                                                .inside(|label, _| {
                                                    *label = egui::Image::from_bytes(
                                                        "bytes://heart.svg",
                                                        HEART,
                                                    );
                                                });
                                        ui.add(image);
                                    });
                                });
                        });
                    });
            });
    }
}
