use crate::app::utils::FilterInput;
use crate::streams::RefStreams;
use egui::Context;
use egui_extras::{Column, TableBuilder};

pub trait TableBase {
    fn new(streams: RefStreams) -> Self;
    fn ui(&mut self, ctx: &Context);
    fn check_filter(&mut self);
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
                        match ($width, $max) {
                            (Some(w), Some(max)) => Column::initial(w).at_least($min).at_most(max).clip($clipped).resizable($column_resizable),
                            (Some(w), None) => Column::initial(w).at_least($min).clip($clipped).resizable($column_resizable),
                            (None, Some(max)) => Column::remainder().at_least($min).at_most(max).clip($clipped).resizable($column_resizable),
                            (None, None) => Column::remainder().at_least($min).clip($clipped).resizable($column_resizable),
                        }
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
