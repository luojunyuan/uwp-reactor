mod app;
mod component;
pub mod controls;
pub mod declaration;
mod ir;
pub mod native;
mod reconcile;
pub mod window;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
pub mod projection {
    include!(concat!(env!("OUT_DIR"), "/windows_ui_xaml.rs"));
}

pub mod application {
    pub use crate::component::{Component, ComponentContext, ViewContext};
}

pub mod xaml {
    pub use crate::projection::*;
}

pub mod prelude {
    pub use crate::application::{Component, ComponentContext, ViewContext};
    pub use crate::controls::{
        Button, CheckBox, Orientation, ProgressBar, Slider, StackPanel, TextBlock, TextBox,
        ToggleSwitch,
    };
    pub use crate::declaration::{IntoViews, View};
    pub use crate::{App, Error, Result};
}

pub use app::App;
pub use component::{Callback, Component, ComponentContext, ViewContext};
pub use controls::{
    Button, CheckBox, Orientation, ProgressBar, Slider, StackPanel, TextBlock, TextBox,
    ToggleSwitch,
};
pub use declaration::{IntoViews, View};
pub use windows_core::{Error, Result};
