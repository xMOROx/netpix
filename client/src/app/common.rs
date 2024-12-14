pub(crate) mod table;
pub(crate) mod types;
mod utils;

use egui_extras::{TableBody, TableRow};
pub use table::*;
pub use types::*;
pub use utils::*;

#[macro_export]
macro_rules! declare_table_struct {
    ($name:ident) => {
        pub struct $name {
            streams: RefStreams,
            filter_input: FilterInput,
            config: TableConfig,
        }
    };
    ($name:ident, $($field:ident: $type:ty),*) => {
        pub struct $name {
            streams: RefStreams,
            filter_input: FilterInput,
            config: TableConfig,
            $($field: $type),*
        }
    };
}

#[macro_export]
macro_rules! impl_table_base {
    (
        $name:ident, $help:expr
        ;
        build_header: |$self:ident, $header:ident| $header_impl:block
        ;
        build_table_body: |$self_body:ident, $body:ident| $body_impl:block
        $(,)?
    ) => {
        impl TableBase for $name {
            fn new(streams: RefStreams) -> Self {
                Self {
                    streams,
                    filter_input: FilterInput::new($help),
                    config: TableConfig::default(),
                }
            }

            fn ui(&mut self, ctx: &egui::Context) {
                if self.filter_input.show(ctx) {
                    self.check_filter();
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.build_table(ui);
                });
            }

            fn check_filter(&mut self) {
                let filter = self.filter_input.get_filter();
                if filter.is_empty() {
                    self.filter_input.set_error(None);
                    return;
                }

                let result = parse_filter(&filter.to_lowercase());
                self.filter_input.set_error(result.err());
            }

            fn build_header($self:&mut Self, $header: &mut TableRow) {
                $header_impl
            }

            fn build_table_body($self_body:&mut Self, $body: TableBody) {
                $body_impl
            }
        }
    };
}
