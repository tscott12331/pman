use eframe::{App, CreationContext, egui::{self, CentralPanel, Color32, Panel}};

const INPUT_BG: Color32 = Color32::from_rgb(0x22, 0x22, 0x22);
const INPUT_BG_H: Color32 = Color32::from_rgb(0x30, 0x30, 0x30);

struct PasswordResult {
    domain: String,
    created: std::time::Instant,
}

#[derive(Default)]
pub struct PManApp {
    search: String,
    password_results: Vec<PasswordResult>,
}

impl PManApp {
    pub fn new(_ctx: &CreationContext<'_>) -> PManApp {
        PManApp::default()
    }

    fn password_card(&self, ui: &mut egui::Ui, pres: &PasswordResult) -> egui::InnerResponse<()> {
        let size = egui::vec2(ui.available_width(), PASSWORD_CARD_HEIGHT);
        let (rect, res) = ui.allocate_exact_size(size, egui::Sense::hover() | egui::Sense::click());

        let bg = if res.hovered() {
            INPUT_BG_H
        } else {
            INPUT_BG
        };

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(*ui.layout())
        );

        egui::Frame::default()
            .fill(bg)
            .show(&mut child_ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(PASSWORD_CARD_HEIGHT);
                    ui.heading(pres.domain.clone());
                });
            }
        )
    }

    fn password_card_area(&self, ui: &mut egui::Ui) -> egui::scroll_area::ScrollAreaOutput<()> {
        egui::scroll_area::ScrollArea::new([false, true])
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    for pass_res in self.password_results.iter() {
                        self.password_card(ui, pass_res);
                    }
            });
        })
    }
    fn controls(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
        let width = ui.available_width();
        ui.horizontal_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .desired_width(INPUT_WIDTH_P*width)
                    .background_color(INPUT_BG)
            );

            let button = ui.add(egui::Button::new("+").fill(INPUT_BG));

            if button.clicked() {
                self.password_results.push(PasswordResult {
                    domain: self.search.clone(),
                    created: std::time::Instant::now(),
                });
            }
        })
    }
}

const INPUT_WIDTH_P: f32 = 0.95;
const INNER_MARGIN: f32 = 50.0;

const PASSWORD_CARD_HEIGHT: f32 = 75.0;
impl App for PManApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        Panel::top("controls")
            .frame(
                egui::Frame::default()
                .inner_margin(INNER_MARGIN)
            )
            .show(ui, |ui| {
                self.controls(ui);
            }
        );

        CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(INNER_MARGIN)
            )
            .show(ui, |ui| {
                self.password_card_area(ui);
            });
    }
}
