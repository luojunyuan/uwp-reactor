use crate::component::Callback;

#[derive(Clone)]
pub struct View(pub(crate) Node);

#[derive(Clone)]
pub(crate) enum Node {
    TextBlock {
        text: String,
        font_size: f64,
    },
    Button {
        content: String,
        on_click: Option<Callback<()>>,
    },
    StackPanel {
        orientation: crate::controls::Orientation,
        spacing: f64,
        children: Vec<View>,
    },
    TextBox {
        text: String,
        placeholder: String,
        on_text_changed: Option<Callback<String>>,
    },
    CheckBox {
        content: String,
        is_checked: bool,
        on_checked_changed: Option<Callback<bool>>,
    },
    ToggleSwitch {
        header: String,
        is_on: bool,
        on_toggled: Option<Callback<bool>>,
    },
    Slider {
        minimum: f64,
        maximum: f64,
        step_frequency: f64,
        value: f64,
        on_value_changed: Option<Callback<f64>>,
    },
    ProgressBar {
        minimum: f64,
        maximum: f64,
        value: f64,
    },
}

impl View {
    pub fn text(text: impl Into<String>) -> Self {
        Self(Node::TextBlock {
            text: text.into(),
            font_size: 14.0,
        })
    }

    pub(crate) fn node(&self) -> &Node {
        &self.0
    }
}

impl From<String> for View {
    fn from(value: String) -> Self {
        Self::text(value)
    }
}

impl From<&str> for View {
    fn from(value: &str) -> Self {
        Self::text(value)
    }
}

pub trait IntoViews {
    fn into_views(self) -> Vec<View>;
}

impl<I, V> IntoViews for I
where
    I: IntoIterator<Item = V>,
    V: Into<View>,
{
    fn into_views(self) -> Vec<View> {
        self.into_iter().map(Into::into).collect()
    }
}
