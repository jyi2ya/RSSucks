use egui::{Image, Margin, RichText, Rounding, Widget};
use uuid::Uuid;

use super::{Builder, Element, ElementType};

pub struct Preview<'a> {
    // rendering previews needs ownership
    elements: &'a Option<Vec<Element>>,
    scroll_area_id: Uuid,
    max_rows: usize,
    break_anywhere: bool,
    overflow_character: Option<char>,
    fulltext: Option<String>,
    max_images_num: usize,
    title: String,
}

impl<'a> From<&Builder<'a>> for Preview<'a> {
    fn from(value: &Builder<'a>) -> Self {
        Preview {
            elements: &value.elements,
            scroll_area_id: Uuid::new_v4(),
            max_rows: value.max_rows,
            break_anywhere: value.break_anywhere,
            overflow_character: value.overflow_character,
            fulltext: value.fulltext.clone(),
            max_images_num: 3,
            title: value.title.to_owned(),
        }
    }
}

impl<'a> Widget for &Preview<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(ui.available_size(), |ui| {
            egui::Frame::none()
                .inner_margin(Margin::same(32.0))
                .outer_margin(Margin::symmetric(
                    if ui.max_rect().width() > 1024.0 {
                        (ui.max_rect().width() - 1024.0) / 2.0
                    } else {
                        0.0
                    },
                    8.0,
                ))
                .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                .rounding(Rounding::ZERO.at_least(10.0))
                .show(ui, |ui: &mut egui::Ui| {
                    // Set the spacing between header and content.
                    ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    // Render title:
                    ui.label(RichText::new(self.title).size(20.0).strong());

                    // Render content:
                    // First, render text.
                    let mut job = egui::text::LayoutJob::single_section(
                        self.fulltext.map_or("No content...".to_owned(), |text| text),
                        egui::TextFormat::default(),
                    );
                    job.wrap = egui::text::TextWrapping {
                        max_rows: self.max_rows,
                        break_anywhere: self.break_anywhere,
                        overflow_character: self.overflow_character,
                        ..Default::default()
                    };
                    ui.label(job);

                    // Then render images.
                    if let Some(elements) = self.elements {
                        let mut images_iter = elements
                            .iter()
                            .filter_map(|element| {
                                if element.typ == ElementType::Image {
                                    element.image_tuple.0.as_ref()
                                } else {
                                    None
                                }
                            })
                            .take(self.max_images_num)
                            .peekable();
                        if images_iter.peek().is_some() {
                            egui::ScrollArea::horizontal()
                                .id_source(self.scroll_area_id)
                                .auto_shrink([false, true])
                                .drag_to_scroll(true)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        images_iter.for_each(|src| {
                                            ui.add(
                                                Image::from(src)
                                                    .fit_to_exact_size(egui::Vec2::new(
                                                        f32::INFINITY, 128.0,
                                                    ))
                                                    .rounding(Rounding::ZERO.at_least(10.0))
                                                    .show_loading_spinner(true),
                                            );
                                        });
                                    });
                                });
                        }
                    }
                    ui.allocate_space(egui::Vec2 {
                        x: ui.max_rect().width(),
                        y: 0.0,
                    });
                });
        })
        .response
    }
}
