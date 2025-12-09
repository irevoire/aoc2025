use aoc::Coord;
use egui::{Color32, RichText, Stroke, Vec2, Vec2b};
use egui_plot::Plot;

fn main() {
    let coords = aoc::parser::lines::<Coord<isize>>().collect::<Vec<_>>();
    let points = coords
        .iter()
        .map(|c| egui_plot::PlotPoint {
            x: c.x as f64,
            y: c.y as f64,
        })
        .collect::<Vec<_>>();

    // let max = coords
    //     .iter()
    //     .flat_map(|left| coords.iter().map(move |right| (left, right)))
    //     .map(|(left, right)| {
    //         ((left.x as usize).abs_diff(right.x as usize) + 1)
    //             * ((left.y as usize).abs_diff(right.y as usize) + 1)
    //     })
    //     .max()
    //     .unwrap();

    let mut rectangles = Vec::with_capacity(points.len() + 1);
    for (a, b) in points
        .iter()
        .flat_map(|left| points.iter().map(move |right| (left, right)))
    {
        let surface = ((a.x as usize).abs_diff(b.x as usize) + 1)
            * ((a.y as usize).abs_diff(b.y as usize) + 1);

        let polygon = vec![
            egui_plot::PlotPoint {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            egui_plot::PlotPoint {
                x: a.x.min(b.x),
                y: a.y.max(b.y),
            },
            egui_plot::PlotPoint {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
            egui_plot::PlotPoint {
                x: a.x.max(b.x),
                y: a.y.min(b.y),
            },
            egui_plot::PlotPoint {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
        ];
        rectangles.push((surface, polygon));
    }
    rectangles.sort_unstable_by_key(|(surface, _)| *surface);
    rectangles.dedup_by_key(|(surface, _)| *surface);
    rectangles.reverse();

    let app = App {
        points,
        rectangles,
        inspecting: 0,
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "aoc day 9",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}

struct App {
    points: Vec<egui_plot::PlotPoint>,
    rectangles: Vec<(usize, Vec<egui_plot::PlotPoint>)>,
    inspecting: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|input| input.key_pressed(egui::Key::ArrowLeft)) {
            self.inspecting = self.inspecting.saturating_sub(1);
        }
        if ctx.input(|input| input.key_pressed(egui::Key::ArrowRight)) {
            self.inspecting += 1;
            if self.inspecting == self.rectangles.len() {
                self.inspecting -= 1;
            }
        }

        // retrieve the current rectangle
        let (surface, points) = &self.rectangles[self.inspecting];

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("Look for the biggest rectangle contained within the large polygon!");
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("Inspecting polygon of surface ".to_string());
            ui.label(
                RichText::new(surface.to_string())
                    .extra_letter_spacing(1.0)
                    .color(Color32::YELLOW)
                    .monospace()
                    .strong(),
            );
            ui.horizontal(|ui| {
                if ui.button("Previous").clicked() {
                    self.inspecting = self.inspecting.saturating_sub(1);
                }
                ui.add(
                    egui::Slider::new(&mut self.inspecting, 0..=self.rectangles.len() - 1)
                        .smart_aim(false)
                        .drag_value_speed(0.1),
                );
                if ui.button("Next").clicked() {
                    self.inspecting += 1;
                    if self.inspecting == self.rectangles.len() {
                        self.inspecting -= 1;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("my_plot")
                .view_aspect(1.0)
                .show_axes(Vec2b::FALSE)
                .show_grid(Vec2b::FALSE)
                .show(ui, |plot_ui| {
                    let background = egui_plot::Polygon::new(
                        format!("{surface}"),
                        egui_plot::PlotPoints::Borrowed(points),
                    )
                    .fill_color(Color32::RED)
                    .stroke(Stroke::NONE);
                    plot_ui.polygon(background);
                    let polygon = egui_plot::Polygon::new(
                        "main polygon",
                        egui_plot::PlotPoints::Borrowed(&self.points),
                    )
                    .fill_color(Color32::DARK_GREEN);
                    plot_ui.polygon(polygon);
                    let target = egui_plot::Polygon::new(
                        format!("{surface}"),
                        egui_plot::PlotPoints::Borrowed(points),
                    );
                    plot_ui.polygon(target);
                    surface
                });
        });
    }
}
