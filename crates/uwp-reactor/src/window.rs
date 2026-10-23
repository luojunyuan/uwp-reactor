use crate::Result;
use crate::xaml::{UIElement, Window};

#[derive(Clone, Debug)]
pub struct WindowHandle(pub(crate) Window);

impl WindowHandle {
    pub fn current() -> Result<Self> {
        Ok(Self(Window::Current()?))
    }

    pub fn close(&self) -> Result<()> {
        self.0.Close()
    }

    pub(crate) fn set_content(&self, content: &UIElement) -> Result<()> {
        self.0.SetContent(content)
    }

    pub(crate) fn activate(&self) -> Result<()> {
        self.0.Activate()
    }
}
