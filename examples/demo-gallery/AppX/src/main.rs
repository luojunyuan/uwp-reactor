#![cfg_attr(windows, windows_subsystem = "windows")]

use uwp_reactor::{
    App, Button, CheckBox, Component, ComponentContext, Orientation, ProgressBar, Slider,
    StackPanel, TextBlock, TextBox, ToggleSwitch, View, ViewContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Page {
    Overview,
    Controls,
    Layout,
    About,
}

#[derive(Clone)]
enum Message {
    Navigate(Page),
    Increment,
    Decrement,
    Reset,
    NameChanged(String),
    FeatureChanged(bool),
    VolumeChanged(f64),
}

struct Gallery {
    page: Page,
    count: i32,
    name: String,
    enabled: bool,
    volume: f64,
}

impl Component for Gallery {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            page: Page::Overview,
            count: 0,
            name: String::new(),
            enabled: true,
            volume: 45.0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Navigate(page) => self.page = page,
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
            Message::NameChanged(name) => self.name = name,
            Message::FeatureChanged(enabled) => self.enabled = enabled,
            Message::VolumeChanged(volume) => self.volume = volume,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let navigation = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children(vec![
                View::from(
                    Button::new()
                        .content("Overview")
                        .on_click(context.message(Message::Navigate(Page::Overview))),
                ),
                View::from(
                    Button::new()
                        .content("Controls")
                        .on_click(context.message(Message::Navigate(Page::Controls))),
                ),
                View::from(
                    Button::new()
                        .content("Layout")
                        .on_click(context.message(Message::Navigate(Page::Layout))),
                ),
                View::from(
                    Button::new()
                        .content("About")
                        .on_click(context.message(Message::Navigate(Page::About))),
                ),
            ]);

        let page = match self.page {
            Page::Overview => self.overview_page(),
            Page::Controls => self.controls_page(context),
            Page::Layout => self.layout_page(),
            Page::About => self.about_page(),
        };

        context.window_frame(
            "UWP Reactor Gallery",
            StackPanel::new()
                .spacing(18.0)
                .children(vec![navigation.into(), page]),
        )
    }
}

impl Gallery {
    fn overview_page(&self) -> View {
        StackPanel::new()
            .spacing(12.0)
            .children(vec![
                View::from(TextBlock::new().text("Overview").font_size(22.0)),
                View::from(TextBlock::new().text("A native Windows.UI.Xaml component gallery.")),
                View::from(TextBlock::new().text(format!("Counter value: {}", self.count))),
                View::from(TextBlock::new().text(format!(
                    "Input preview: {}",
                    if self.name.is_empty() {
                        "Type a name on the Controls page"
                    } else {
                        &self.name
                    }
                ))),
                View::from(TextBlock::new().text(format!(
                    "Switch state: {}",
                    if self.enabled { "On" } else { "Off" }
                ))),
                View::from(TextBlock::new().text(format!("Slider value: {:.0}%", self.volume))),
            ])
            .into()
    }

    fn controls_page(&self, context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(12.0)
            .children(vec![
                View::from(TextBlock::new().text("Basic controls").font_size(22.0)),
                View::from(TextBlock::new().text(format!("Current value: {}", self.count))),
                View::from(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children(vec![
                            View::from(
                                Button::new()
                                    .content("Decrease")
                                    .on_click(context.message(Message::Decrement)),
                            ),
                            View::from(
                                Button::new()
                                    .content("Increase")
                                    .on_click(context.message(Message::Increment)),
                            ),
                            View::from(
                                Button::new()
                                    .content("Reset")
                                    .on_click(context.message(Message::Reset)),
                            ),
                        ]),
                ),
                View::from(
                    TextBox::new()
                        .text(self.name.clone())
                        .placeholder_text("Enter a name")
                        .on_text_changed(context.callback(Message::NameChanged)),
                ),
                View::from(TextBlock::new().text(format!(
                    "Hello{}{}",
                    if self.name.is_empty() { "" } else { ", " },
                    self.name
                ))),
                View::from(
                    CheckBox::new()
                        .content("Enable feature")
                        .is_checked(self.enabled)
                        .on_checked_changed(context.callback(Message::FeatureChanged)),
                ),
                View::from(
                    ToggleSwitch::new()
                        .header("Feature status")
                        .is_on(self.enabled)
                        .on_toggled(context.callback(Message::FeatureChanged)),
                ),
                View::from(TextBlock::new().text(format!("Progress: {:.0}%", self.volume))),
                View::from(
                    Slider::new()
                        .minimum(0.0)
                        .maximum(100.0)
                        .step_frequency(1.0)
                        .value(self.volume)
                        .on_value_changed(context.callback(Message::VolumeChanged)),
                ),
                View::from(
                    ProgressBar::new()
                        .minimum(0.0)
                        .maximum(100.0)
                        .value(self.volume),
                ),
            ])
            .into()
    }

    fn layout_page(&self) -> View {
        StackPanel::new()
            .spacing(12.0)
            .children(vec![
                View::from(TextBlock::new().text("Layout").font_size(22.0)),
                View::from(
                    TextBlock::new()
                        .text("Vertical and horizontal stack panels compose native controls."),
                ),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children(vec![
                        View::from(TextBlock::new().text("First group")),
                        View::from(TextBlock::new().text("Second group")),
                        View::from(TextBlock::new().text("Third group")),
                    ])
                    .into(),
                View::from(TextBlock::new().text(format!("The shared counter is {}.", self.count))),
            ])
            .into()
    }

    fn about_page(&self) -> View {
        StackPanel::new()
            .spacing(12.0)
            .children(vec![
                View::from(TextBlock::new().text("About this gallery").font_size(22.0)),
                View::from(
                    TextBlock::new()
                        .text("Components render directly to the Windows.UI.Xaml runtime."),
                ),
                View::from(TextBlock::new().text("Package the app with winapp when ready.")),
            ])
            .into()
    }
}

fn main() {
    App::run_component::<Gallery>(()).expect("failed to start the UWP demo gallery");
}
