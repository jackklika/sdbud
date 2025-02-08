#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{vec2, Color32, Frame, Id, Ui};
use crate::music::MusicState;

pub fn render(state: MusicState) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(move |_cc| Ok(Box::new(MyApp::new(state)))),
    )
}

pub struct DragAndDropDemo {
    /// columns with items
    columns: Vec<Vec<String>>,
}


struct MyApp {
    name: String,
    age: u32,
    drag_and_drop: DragAndDropDemo,
    state: MusicState,
}


/// What is being dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}



impl MyApp {
    pub fn new(state: MusicState) -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            drag_and_drop: DragAndDropDemo {
                columns: vec![
                    vec!["Item 1".to_owned(), "Item 2".to_owned(), "Item 3".to_owned()],
                    vec!["Item 4".to_owned(), "Item 5".to_owned()],
                    vec!["Item 6".to_owned(), "Item 7".to_owned()],
                ],
            },
            state,
        }
    }
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            drag_and_drop: DragAndDropDemo {
                columns: vec![
                    vec!["Item 1".to_owned(), "Item 2".to_owned(), "Item 3".to_owned()],
                    vec!["Item 4".to_owned(), "Item 5".to_owned()],
                    vec!["Item 6".to_owned(), "Item 7".to_owned()],
                ],
            },
            state: MusicState::default(),
        }
    }
}


pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, state: &MusicState);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.heading("Drag and Drop Demo");
            self.drag_and_drop.ui(ui, &self.state);

        });
    }
}

impl View for DragAndDropDemo {
    fn ui(&mut self, ui: &mut Ui, state: &MusicState) {
        ui.label("This is a simple example of drag-and-drop in egui.");
        ui.label("Drag items between columns.");

        self.columns[0] = state.albums_1.clone();

        // If there is a drop, store the location of the item being dragged, and the destination for the drop.
        let mut from = None;
        let mut to = None;

        ui.columns(self.columns.len(), |uis| {
            for (col_idx, column) in self.columns.clone().into_iter().enumerate() {
                let ui = &mut uis[col_idx];

                let frame = Frame::default().inner_margin(4.0);

                let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                    ui.set_min_size(vec2(64.0, 100.0));
                    for (row_idx, item) in column.iter().enumerate() {
                        let item_id = Id::new(("my_drag_and_drop_demo", col_idx, row_idx));
                        let item_location = Location {
                            col: col_idx,
                            row: row_idx,
                        };
                        let response = ui
                            .dnd_drag_source(item_id, item_location, |ui| {
                                ui.label(item);
                            })
                            .response;

                        // Detect drops onto this item:
                        if let (Some(pointer), Some(hovered_payload)) = (
                            ui.input(|i| i.pointer.interact_pos()),
                            response.dnd_hover_payload::<Location>(),
                        ) {
                            let rect = response.rect;

                            // Preview insertion:
                            let stroke = egui::Stroke::new(1.0, Color32::WHITE);
                            let insert_row_idx = if *hovered_payload == item_location {
                                // We are dragged onto ourselves
                                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                                row_idx
                            } else if pointer.y < rect.center().y {
                                // Above us
                                ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                row_idx
                            } else {
                                // Below us
                                ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                row_idx + 1
                            };

                            if let Some(dragged_payload) = response.dnd_release_payload() {
                                // The user dropped onto this item.
                                from = Some(dragged_payload);
                                to = Some(Location {
                                    col: col_idx,
                                    row: insert_row_idx,
                                });
                            }
                        }
                    }
                });

                if let Some(dragged_payload) = dropped_payload {
                    // The user dropped onto the column, but not on any one item.
                    from = Some(dragged_payload);
                    to = Some(Location {
                        col: col_idx,
                        row: usize::MAX, // Inset last
                    });
                }
            }
        });

        if let (Some(from), Some(mut to)) = (from, to) {
            if from.col == to.col {
                // Dragging within the same column.
                // Adjust row index if we are re-ordering:
                to.row -= (from.row < to.row) as usize;
            }

            let item = self.columns[from.col].remove(from.row);

            let column = &mut self.columns[to.col];
            to.row = to.row.min(column.len());
            column.insert(to.row, item);
        }

    }
}
