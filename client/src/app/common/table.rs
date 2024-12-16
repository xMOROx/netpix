use crate::app::utils::FilterInput;
use crate::streams::RefStreams;
use egui::Context;
use egui_extras::{Column, TableBuilder};

pub trait TableBase {
    fn new(streams: RefStreams) -> Self;
    fn ui(&mut self, ctx: &Context);
    fn check_filter(&mut self);
    fn build_header(&mut self, header: &mut egui_extras::TableRow);
    fn build_table_body(&mut self, body: egui_extras::TableBody);
}

#[macro_export]
macro_rules! define_column {
    ($width:expr, $min:expr, $max:expr, $clipped:expr, $resizable:expr) => {
        match ($width, $max) {
            (Some(w), Some(max)) => Column::initial(w)
                .at_least($min)
                .at_most(max)
                .clip($clipped)
                .resizable($resizable),
            (Some(w), None) => Column::initial(w)
                .at_least($min)
                .clip($clipped)
                .resizable($resizable),
            (None, Some(max)) => Column::remainder()
                .at_least($min)
                .at_most(max)
                .clip($clipped)
                .resizable($resizable),
            (None, None) => Column::remainder()
                .at_least($min)
                .clip($clipped)
                .resizable($resizable),
        }
    };
}

#[macro_export]
macro_rules! declare_table {
    ($table:ident, $filter:ty, {
        $(height($height:expr))?
        ;
        $(striped($striped:expr))?
        ;
        $(resizable($resizable:expr))?
        ;
        $(stick_to_bottom($stick_to_bottom:expr))?
        ;
        columns(
            $( column($width:expr, $min:expr, $max:expr, $clipped:expr, $column_resizable:expr) ),* $(,)?
        )
        $(,)?
    }) => {
        impl $table {
            fn build_table(&mut self, ui: &mut egui::Ui) {
                let mut builder = TableBuilder::new(ui)
                    .striped($(($striped))?)
                    .resizable($(($resizable))?)
                    .stick_to_bottom($(($stick_to_bottom))?);

                $(
                    builder = builder.column(
                        define_column!($width, $min, $max, $clipped, $column_resizable)
                    );
                )*

                builder
                    .header($(($height))?, |mut header| {
                        self.build_header(&mut header);
                    })
                    .body(|body| {
                        self.build_table_body(body);
                    });
            }
        }
    };
}
