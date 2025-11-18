use crate::streams::RefStreams;
use egui::Context;
use ewebsock::WsSender;
use std::any::Any;

pub trait TableBase: Any {
    fn new(streams: RefStreams, ws_sender: WsSender) -> Self
    where
        Self: Sized;
    fn ui(&mut self, ctx: &Context);
    fn check_filter(&mut self);
    fn build_header(&mut self, header: &mut egui_extras::TableRow);
    fn build_table_body(&mut self, body: egui_extras::TableBody);
    fn table_id(&self) -> &'static str;
    fn table_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct TableRegistry {
    tables: Vec<Box<dyn TableBase>>,
}

impl TableRegistry {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn register<T: TableBase + 'static>(&mut self, streams: RefStreams, ws_sender: WsSender) {
        self.tables.push(Box::new(T::new(streams, ws_sender)));
    }

    pub fn get_table(&self, id: &str) -> Option<&dyn TableBase> {
        self.tables
            .iter()
            .find(|t| t.table_id() == id)
            .map(|t| t.as_ref())
    }

    pub fn get_table_mut(&mut self, id: &str) -> Option<&mut dyn TableBase> {
        self.tables
            .iter_mut()
            .find(|t| t.table_id() == id)
            .map(|t| t.as_mut())
    }
}
