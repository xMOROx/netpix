use crate::{app::utils::FilterInput, streams::RefStreams};
use egui::Context;
use egui_extras::{Column, TableBuilder};

pub trait TableBase {
    fn new(streams: RefStreams) -> Self;
    fn ui(&mut self, ctx: &Context);
    fn check_filter(&mut self);
    fn build_header(&mut self, header: &mut egui_extras::TableRow);
    fn build_table_body(&mut self, body: egui_extras::TableBody);
}
