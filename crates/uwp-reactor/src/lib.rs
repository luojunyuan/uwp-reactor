//! A small application API over the current Windows Reactor runtime.
//!
//! The runtime and controls are sourced from the `reference/windows-rs` submodule so the
//! workspace builds against one pinned Windows API revision.

mod app;

pub mod application {
    pub use windows_reactor::{
        Application, ApplicationContext, ApplicationView, ApplicationViewContext,
    };
}

pub mod component {
    pub use windows_reactor::{
        CancellationToken, Component, ComponentCompletion, ComponentContext, ComponentSender,
        ComponentTask, ComponentTimer, Context, EffectKey, LocalSender, ViewContext, component,
        provide,
    };
}

pub mod controls {
    pub use windows_reactor::{
        AppBarButton, AppBarSeparator, Border, Button, CheckBox, ComboBox, ContentDialog, Grid,
        HyperlinkButton, NavigationView, NumberBox, ProgressBar, ScrollView, Slider, StackPanel,
        TextBlock, TextBox, ToggleSwitch, View,
    };
}

pub mod declaration {
    pub use windows_reactor::{IntoViews, Key, KeyedView, View, keyed};
}

pub mod window {
    pub use windows_reactor::{WindowHandle, WindowTitleBarHeight};
}

pub mod native {
    pub use windows_reactor::{AppCallback, AppContext, AppProxy, ScreenPoint};
}

pub mod reconcile {
    pub use windows_reactor::{AnyElement, CompatibleElementRef, ElementRef, ReferenceElement};
}

pub mod runtime {
    pub use windows_reactor::*;
}

pub mod prelude {
    pub use crate::application::*;
    pub use crate::component::*;
    pub use crate::controls::{
        Button, CheckBox, ProgressBar, Slider, StackPanel, TextBlock, TextBox, ToggleSwitch, View,
    };
    pub use crate::{App, Error, Result};
}

pub use app::App;
pub use component::{Component, ComponentContext, ViewContext};
pub use controls::{
    Button, CheckBox, ProgressBar, Slider, StackPanel, TextBlock, TextBox, ToggleSwitch, View,
};
pub use windows_core::{Error, Result};
