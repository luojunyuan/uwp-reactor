use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::component::{Component, ComponentContext};
use crate::ir::MountedNode;
use crate::window::WindowHandle;
use crate::{Error, Result};

pub struct ApplicationRuntime;

pub(crate) struct Runtime<C: Component> {
    pub(crate) input: C::Input,
    pub(crate) component: C,
    pub(crate) window: WindowHandle,
    pub(crate) root: Option<MountedNode>,
}

impl<C: Component> Runtime<C> {
    fn new(input: C::Input, component: C, window: WindowHandle) -> Self {
        Self {
            input,
            component,
            window,
            root: None,
        }
    }
}

pub(crate) fn dispatch<C: Component>(runtime: &Weak<RefCell<Runtime<C>>>, message: C::Message) {
    let Some(runtime) = runtime.upgrade() else {
        return;
    };
    {
        let mut runtime = runtime.borrow_mut();
        runtime
            .component
            .update(message, &ComponentContext::<C>::new());
    }
    if let Err(error) = crate::reconcile::render(&runtime) {
        eprintln!("uwp-reactor render failed: {error}");
    }
}

impl ApplicationRuntime {
    pub(crate) fn run_component<C: Component>(input: C::Input) -> Result<()> {
        crate::app::start::<C>(input)
    }

    pub(crate) fn launch<C: Component>(input: C::Input) -> Result<Rc<RefCell<Runtime<C>>>> {
        let component = C::create(&input, &ComponentContext::<C>::new());
        let window = WindowHandle::current()?;
        let runtime = Rc::new(RefCell::new(Runtime::new(input, component, window.clone())));
        crate::reconcile::render(&runtime)?;
        window.activate()?;
        Ok(runtime)
    }
}

pub(crate) fn startup_error() -> Error {
    Error::new(
        windows_core::HRESULT(0x80004005u32 as i32),
        "UWP application startup callback was not installed",
    )
}
