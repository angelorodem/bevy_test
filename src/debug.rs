use bevy::prelude::*;
use bevy_editor_pls::{editor_window::EditorWindow, prelude::*};
use bevy_egui::*;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use std::collections::VecDeque;

#[derive(Resource)]
pub struct PlayerSpeedHistory {
    pub history: VecDeque<f32>,
    pub time_since_last_update: f32,
    pub interval: f32,
    pub buffer_capacity: usize,
}

impl PlayerSpeedHistory {
    pub fn log(&mut self, speed: f32, delta_time: f32) {
        if self.time_since_last_update > self.interval {
            self.time_since_last_update = 0.0;
            self.history.push_back(speed);
            if self.history.len() > self.buffer_capacity {
                self.history.pop_front();
            }
        } else {
            self.time_since_last_update += delta_time;
        }
    }
}

pub struct GuiDebugPlugin {
    pub buffer_capacity: usize,
    pub interval: f32,
}

impl Default for GuiDebugPlugin {
    fn default() -> Self {
        Self {
            buffer_capacity: 100,
            interval: 0.1,
        }
    }
}

impl Plugin for GuiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeedHistory {
            history: VecDeque::with_capacity(self.buffer_capacity),
            time_since_last_update: 0.0,
            interval: self.interval,
            buffer_capacity: self.buffer_capacity,
        })
        .add_plugins(EditorPlugin::default())
        .add_editor_window::<PlotEditorWindow>();
    }
}

pub struct PlotEditorWindow;
#[derive(Default)]
pub struct PlotEditorWindowState;

impl EditorWindow for PlotEditorWindow {
    type State = PlotEditorWindowState;

    const NAME: &'static str = "Plot Editor";

    // fn viewport_toolbar_ui(
    //     world: &mut World,
    //     cx: bevy_editor_pls::editor_window::EditorWindowContext,
    //     ui: &mut egui::Ui,
    // ) {
    //     Self::ui(world, cx, ui);
    // }

    fn ui(
        world: &mut World,
        _cx: bevy_editor_pls::editor_window::EditorWindowContext,
        ui: &mut egui::Ui,
    ) {
        let player_history = world.get_resource::<PlayerSpeedHistory>();
        if let Some(player_history) = player_history {
            let my_plot = Plot::new("Player Speed History")
                .legend(Legend::default())
                .clamp_grid(true)
                .auto_bounds_x()
                .auto_bounds_y();

            // let's create a dummy line in the plot
            let graph: Vec<[f64; 2]> = player_history
                .history
                .iter()
                .enumerate()
                .map(|(i, &y)| [i as f64, y as f64])
                .collect();

            my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(graph)).name("Player Speed"));
            });
        }
    }
}
