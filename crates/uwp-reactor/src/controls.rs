use crate::component::Callback;
use crate::declaration::{Node, View};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Default)]
pub struct TextBlock {
    text: String,
    font_size: f64,
}

impl TextBlock {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            font_size: 14.0,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = font_size;
        self
    }
}

impl From<TextBlock> for View {
    fn from(value: TextBlock) -> Self {
        Self(Node::TextBlock {
            text: value.text,
            font_size: value.font_size,
        })
    }
}

#[derive(Clone, Default)]
pub struct Button {
    content: String,
    on_click: Option<Callback<()>>,
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn on_click(mut self, callback: Callback<()>) -> Self {
        self.on_click = Some(callback);
        self
    }
}

impl From<Button> for View {
    fn from(value: Button) -> Self {
        Self(Node::Button {
            content: value.content,
            on_click: value.on_click,
        })
    }
}

#[derive(Clone, Default)]
pub struct StackPanel {
    orientation: Orientation,
    spacing: f64,
    children: Vec<View>,
}

impl StackPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn children<I, V>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<View>,
    {
        self.children = children.into_iter().map(Into::into).collect();
        self
    }
}

impl From<StackPanel> for View {
    fn from(value: StackPanel) -> Self {
        Self(Node::StackPanel {
            orientation: value.orientation,
            spacing: value.spacing,
            children: value.children,
        })
    }
}

#[derive(Clone, Default)]
pub struct TextBox {
    text: String,
    placeholder: String,
    on_text_changed: Option<Callback<String>>,
}

impl TextBox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn placeholder_text(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn on_text_changed(mut self, callback: Callback<String>) -> Self {
        self.on_text_changed = Some(callback);
        self
    }
}

impl From<TextBox> for View {
    fn from(value: TextBox) -> Self {
        Self(Node::TextBox {
            text: value.text,
            placeholder: value.placeholder,
            on_text_changed: value.on_text_changed,
        })
    }
}

#[derive(Clone, Default)]
pub struct CheckBox {
    content: String,
    is_checked: bool,
    on_checked_changed: Option<Callback<bool>>,
}

impl CheckBox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn is_checked(mut self, is_checked: bool) -> Self {
        self.is_checked = is_checked;
        self
    }

    pub fn on_checked_changed(mut self, callback: Callback<bool>) -> Self {
        self.on_checked_changed = Some(callback);
        self
    }
}

impl From<CheckBox> for View {
    fn from(value: CheckBox) -> Self {
        Self(Node::CheckBox {
            content: value.content,
            is_checked: value.is_checked,
            on_checked_changed: value.on_checked_changed,
        })
    }
}

#[derive(Clone, Default)]
pub struct ToggleSwitch {
    header: String,
    is_on: bool,
    on_toggled: Option<Callback<bool>>,
}

impl ToggleSwitch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = header.into();
        self
    }

    pub fn is_on(mut self, is_on: bool) -> Self {
        self.is_on = is_on;
        self
    }

    pub fn on_toggled(mut self, callback: Callback<bool>) -> Self {
        self.on_toggled = Some(callback);
        self
    }
}

impl From<ToggleSwitch> for View {
    fn from(value: ToggleSwitch) -> Self {
        Self(Node::ToggleSwitch {
            header: value.header,
            is_on: value.is_on,
            on_toggled: value.on_toggled,
        })
    }
}

#[derive(Clone)]
pub struct Slider {
    minimum: f64,
    maximum: f64,
    step_frequency: f64,
    value: f64,
    on_value_changed: Option<Callback<f64>>,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            minimum: 0.0,
            maximum: 100.0,
            step_frequency: 1.0,
            value: 0.0,
            on_value_changed: None,
        }
    }
}

impl Slider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn minimum(mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn maximum(mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self
    }

    pub fn step_frequency(mut self, step_frequency: f64) -> Self {
        self.step_frequency = step_frequency;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn on_value_changed(mut self, callback: Callback<f64>) -> Self {
        self.on_value_changed = Some(callback);
        self
    }
}

impl From<Slider> for View {
    fn from(value: Slider) -> Self {
        Self(Node::Slider {
            minimum: value.minimum,
            maximum: value.maximum,
            step_frequency: value.step_frequency,
            value: value.value,
            on_value_changed: value.on_value_changed,
        })
    }
}

#[derive(Clone)]
pub struct ProgressBar {
    minimum: f64,
    maximum: f64,
    value: f64,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            minimum: 0.0,
            maximum: 100.0,
            value: 0.0,
        }
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn minimum(mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn maximum(mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

impl From<ProgressBar> for View {
    fn from(value: ProgressBar) -> Self {
        Self(Node::ProgressBar {
            minimum: value.minimum,
            maximum: value.maximum,
            value: value.value,
        })
    }
}
