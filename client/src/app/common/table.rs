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
        $(column_with_width($width_1:expr, $min_1:expr, $max_1:expr, $clipped_1:expr)),*
        ;
        $(column_without_max($width_2:expr, $min_2:expr, $clipped_2:expr)),*
        ;
        $(reminder_column($min_3:expr, $max_3:expr, $clipped_3:expr)),*
        ;
        $(reminder_column_with_min($min_4:expr, $clipped_4:expr)),*
        $(,)?
    }) => {
        impl $table {
            fn build_table(&mut self, ui: &mut egui::Ui) {
                TableBuilder::new(ui)
                    .striped($(($striped))?)
                    .resizable($(($resizable))?)
                    .stick_to_bottom($(($stick_to_bottom))?)
                    $(
                        .column(Column::initial($width_1)
                            .at_least($min_1).
                            at_most($max_1)
                            .clip($clipped_1)
                        )
                    )*
                    $(
                        .column(Column::initial($width_2)
                            .at_least($min_2)
                            .clip($clipped_2)
                        )
                    )*
                    $(
                        .column(Column::remainder().at_least($min_3).at_most($max_3).clip($clipped_3))
                    )*
                    $(
                        .column(Column::remainder().at_least($min_4).clip($clipped_4))
                    )*
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
