use crate::xaml::UIElement;
use windows_core::EventRevoker;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    TextBlock,
    Button,
    StackPanel,
    TextBox,
    CheckBox,
    ToggleSwitch,
    Slider,
    ProgressBar,
}

pub struct MountedNode {
    pub(crate) kind: NodeKind,
    pub(crate) element: UIElement,
    pub(crate) children: Vec<MountedNode>,
    pub(crate) event_revokers: Vec<EventRevoker>,
}

impl MountedNode {
    pub(crate) fn new(kind: NodeKind, element: UIElement) -> Self {
        Self {
            kind,
            element,
            children: Vec::new(),
            event_revokers: Vec::new(),
        }
    }
}
