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
use windows::Win32::{
    combaseapi::{CoInitializeEx, CoUninitialize},
    objbase::COINIT_MULTITHREADED,
};
use windows_core::{Ref, implement};

struct MtaApartment(PhantomData<Rc<()>>);

impl MtaApartment {
    fn init() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;
        }
        Ok(Self(PhantomData))
    }
}

impl Drop for MtaApartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

thread_local! {
    static APPLICATION_RUNNING: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn start<C: Component>(input: C::Input) -> Result<()> {
    let _apartment = MtaApartment::init()?;
    if APPLICATION_RUNNING.with(|running| running.replace(true)) {
        return Err(Error::new(
            windows_core::HRESULT(0x80004005u32 as i32),
            "an application is already running on this thread",
        ));
    }

    let input = RefCell::new(Some(input));
    let startup_error = Rc::new(RefCell::new(None));
    let callback_error = Rc::clone(&startup_error);
    let callback = ApplicationInitializationCallback::new(move |_| {
        let input = input.borrow_mut().take();
        let result = input
            .ok_or_else(crate::native::startup_error)
            .and_then(|input| Application::compose(UwpApplication::<C>::new(input)).map(|_| ()));
        match result {
            Ok(()) => {}
            Err(error) => {
                callback_error.borrow_mut().replace(error);
            }
        }
    });
    let start_result = Application::Start(&callback);
    APPLICATION_RUNNING.with(|running| running.set(false));
    if let Some(error) = startup_error.borrow_mut().take() {
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
