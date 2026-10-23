use crate::{Component, Result, View};

/// Entry point for view-based, component-based, and application-based programs.
#[derive(Clone, Copy, Debug, Default)]
pub struct App;

impl App {
    /// Runs a single declarative view in a native window.
    pub fn run(view: impl Into<View>) -> Result<()> {
        windows_reactor::App::run(view)
    }

    /// Runs a typed component with its own state and message loop.
    pub fn run_component<C: Component>(input: C::Input) -> Result<()> {
        windows_reactor::App::run_component::<C>(input)
    }

    /// Runs an application that manages its own windows and application-level state.
    pub fn run_application<A: crate::application::Application>(input: A::Input) -> Result<()> {
        windows_reactor::App::run_application::<A>(input)
    }
}
