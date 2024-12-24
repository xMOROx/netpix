#[macro_export]
macro_rules! define_filter_context {
    ($name:ident, $($field:ident: $type:ty),*) => {
        pub struct $name<'a> {
            $(pub $field: &'a $type),*
        }
    };
}

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
   (
        $name:ident; $($field_name:ident : $field_type:ty),* $(,)?; $help:expr
        ;
        ui: |$ui_self:ident, $ctx:ident| $ui_body:block
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
                    $(
                        $field_name: <$field_type>::default(),
                    )*
                }
            }
            fn ui(&mut $ui_self, $ctx: &egui::Context) {
                $ui_body
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
