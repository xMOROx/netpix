use crate::streams::RefStreams;
use egui::Context;
use ewebsock::WsSender;
use std::any::Any;

pub trait PlotBase: Any {
    fn new(streams: RefStreams, ws_sender: WsSender) -> Self
    where
        Self: Sized;
    fn ui(&mut self, ctx: &Context);
    fn plot_id(&self) -> &'static str;
    fn plot_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct PlotRegistry {
    plots: Vec<Box<dyn PlotBase>>,
}

impl PlotRegistry {
    pub fn new() -> Self {
        Self { plots: Vec::new() }
    }

    pub fn register<T: PlotBase + 'static>(&mut self, streams: RefStreams, ws_sender: WsSender) {
        self.plots.push(Box::new(T::new(streams, ws_sender)));
    }

    pub fn get_plot(&self, id: &str) -> Option<&dyn PlotBase> {
        self.plots
            .iter()
            .find(|p| p.plot_id() == id)
            .map(|p| p.as_ref())
    }

    pub fn get_plot_mut(&mut self, id: &str) -> Option<&mut dyn PlotBase> {
        self.plots
            .iter_mut()
            .find(|p| p.plot_id() == id)
            .map(|p| p.as_mut())
    }
}
