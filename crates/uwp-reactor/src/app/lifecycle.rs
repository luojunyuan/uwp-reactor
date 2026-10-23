use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;

use crate::component::Component;
use crate::native::{ApplicationRuntime, Runtime};
use crate::xaml::{
    Application, ApplicationInitializationCallback, CachedFileUpdaterActivatedEventArgs,
    FileActivatedEventArgs, FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs,
    IActivatedEventArgs, IApplicationOverrides, IApplicationOverrides_Impl,
    LaunchActivatedEventArgs, SearchActivatedEventArgs, ShareTargetActivatedEventArgs,
    WindowCreatedEventArgs,
};
use crate::{Error, Result};
use windows_core::{Ref, implement};

type ApplicationFactory = Box<dyn FnOnce() -> Result<Application>>;

thread_local! {
    static APPLICATION_RUNNING: Cell<bool> = const { Cell::new(false) };
    static APPLICATION_FACTORY: RefCell<Option<ApplicationFactory>> = const { RefCell::new(None) };
    static APPLICATION_INSTANCE: RefCell<Option<Application>> = const { RefCell::new(None) };
    static STARTUP_ERROR: RefCell<Option<Error>> = const { RefCell::new(None) };
}

pub(crate) fn start<C: Component>(input: C::Input) -> Result<()> {
    let _apartment = windows_core::init_sta()?;
    if APPLICATION_RUNNING.with(|running| running.replace(true)) {
        return Err(Error::new(
            windows_core::HRESULT(0x80004005u32 as i32),
            "an application is already running on this thread",
        ));
    }
    APPLICATION_INSTANCE.with(|application| application.replace(None));
    STARTUP_ERROR.with(|error| error.replace(None));

    let previous = APPLICATION_FACTORY.with(|factory| {
        factory.replace(Some(Box::new(move || {
            Application::compose(UwpApplication::<C>::new(input))
        })))
    });
    if previous.is_some() {
        APPLICATION_FACTORY.with(|factory| factory.replace(previous));
        APPLICATION_RUNNING.with(|running| running.set(false));
        return Err(Error::new(
            windows_core::HRESULT(0x80004005u32 as i32),
            "an application is already starting on this thread",
        ));
    }

    let callback = ApplicationInitializationCallback::new(|_| {
        let result = APPLICATION_FACTORY.with(|factory| {
            let factory = factory
                .borrow_mut()
                .take()
                .ok_or_else(crate::native::startup_error)?;
            factory()
        });
        match result {
            Ok(application) => {
                APPLICATION_INSTANCE.with(|instance| instance.replace(Some(application)));
            }
            Err(error) => {
                STARTUP_ERROR.with(|startup_error| startup_error.replace(Some(error)));
            }
        }
    });
    let start_result = Application::Start(&callback);
    APPLICATION_FACTORY.with(|factory| drop(factory.borrow_mut().take()));
    APPLICATION_INSTANCE.with(|application| drop(application.borrow_mut().take()));
    APPLICATION_RUNNING.with(|running| running.set(false));
    if let Some(error) = STARTUP_ERROR.with(|startup_error| startup_error.borrow_mut().take()) {
        return Err(error);
    }
    start_result
}

#[implement(IApplicationOverrides)]
struct UwpApplication<C: Component> {
    input: RefCell<Option<C::Input>>,
    runtime: RefCell<Option<Rc<RefCell<Runtime<C>>>>>,
    marker: PhantomData<fn() -> C>,
}

impl<C: Component> UwpApplication<C> {
    fn new(input: C::Input) -> Self {
        Self {
            input: RefCell::new(Some(input)),
            runtime: RefCell::new(None),
            marker: PhantomData,
        }
    }
}

#[allow(non_snake_case)]
impl<C: Component> IApplicationOverrides_Impl for UwpApplication_Impl<C> {
    fn OnActivated(&self, _: Ref<IActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnLaunched(&self, _: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        let input = self
            .input
            .borrow_mut()
            .take()
            .ok_or_else(crate::native::startup_error)?;
        let runtime = ApplicationRuntime::launch::<C>(input)?;
        self.runtime.replace(Some(runtime));
        Ok(())
    }

    fn OnFileActivated(&self, _: Ref<FileActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnSearchActivated(&self, _: Ref<SearchActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnShareTargetActivated(&self, _: Ref<ShareTargetActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnFileOpenPickerActivated(&self, _: Ref<FileOpenPickerActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnFileSavePickerActivated(&self, _: Ref<FileSavePickerActivatedEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnCachedFileUpdaterActivated(
        &self,
        _: Ref<CachedFileUpdaterActivatedEventArgs>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnWindowCreated(&self, _: Ref<WindowCreatedEventArgs>) -> Result<()> {
        Ok(())
    }
}
