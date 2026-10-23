#![cfg_attr(windows, windows_subsystem = "windows")]

use std::rc::Rc;

use uwp_reactor::{
    App, Button, CheckBox, Component, ComponentContext, ProgressBar, Slider, StackPanel, TextBlock,
    TextBox, ToggleSwitch, View, ViewContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Page {
    Home,
    Controls,
    Layout,
    About,
}

#[derive(Clone, Debug)]
enum Message {
    Navigate(Page),
    Increment,
    Decrement,
    Reset,
    NameChanged(String),
    SetEnabled(bool),
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

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            page: Page::Home,
            count: 0,
            name: String::new(),
            enabled: true,
            volume: 45.0,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Navigate(page) => self.page = page,
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
            Message::NameChanged(name) => self.name = name,
            Message::SetEnabled(enabled) => self.enabled = enabled,
            Message::VolumeChanged(volume) => self.volume = volume,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let navigation = StackPanel::new().spacing(8.0).children((
            Button::new()
                .content("Overview")
                .on_click(context.message(Message::Navigate(Page::Home))),
            Button::new()
                .content("Controls")
                .on_click(context.message(Message::Navigate(Page::Controls))),
            Button::new()
                .content("Layout")
                .on_click(context.message(Message::Navigate(Page::Layout))),
            Button::new()
                .content("About")
                .on_click(context.message(Message::Navigate(Page::About))),
        ));

        let page = match self.page {
            Page::Home => self.home_page(),
            Page::Controls => self.controls_page(context),
            Page::Layout => self.layout_page(),
            Page::About => self.about_page(),
        };

        context.window_frame(
            "UWP Reactor Gallery",
            StackPanel::new().spacing(18.0).children((
                TextBlock::new().text("UWP Reactor").font_size(30.0),
                TextBlock::new()
                    .text("WinUI component gallery")
                    .font_size(16.0),
                navigation,
                page,
            )),
        )
    }
}

impl Gallery {
    fn home_page(&self) -> StackPanel {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("Overview").font_size(24.0),
            TextBlock::new().text("A compact gallery of native controls and declarative layouts."),
            TextBlock::new().text(format!("Counter value: {}", self.count)),
            TextBlock::new().text(format!(
                "Input preview: {}",
                if self.name.is_empty() {
                    "Type a name on the Controls page".to_string()
                } else {
                    self.name.clone()
                }
            )),
            TextBlock::new().text(format!(
                "Switch state: {}",
                if self.enabled { "On" } else { "Off" }
            )),
            TextBlock::new().text(format!("Slider value: {:.0}%", self.volume)),
        ))
    }

    fn controls_page(&self, context: &mut ViewContext<Self>) -> StackPanel {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("Basic controls").font_size(24.0),
            TextBlock::new().text(format!("Current value: {}", self.count)),
            StackPanel::new().spacing(8.0).children((
                Button::new()
                    .content("Decrease")
                    .on_click(context.message(Message::Decrement)),
                Button::new()
                    .content("Increase")
                    .on_click(context.message(Message::Increment)),
                Button::new()
                    .content("Reset")
                    .on_click(context.message(Message::Reset)),
            )),
            TextBox::new()
                .text(self.name.clone())
                .placeholder_text("Enter a name")
                .on_text_changed(
                    context.callback(|value: Rc<str>| Message::NameChanged(value.to_string())),
                ),
            TextBlock::new().text(format!(
                "Hello{}{}",
                if self.name.is_empty() { "" } else { ", " },
                self.name
            )),
            CheckBox::new()
                .content("Enable feature")
                .is_checked(self.enabled)
                .on_is_checked_changed(
                    context.callback(|value: Option<bool>| {
                        Message::SetEnabled(value.unwrap_or(false))
                    }),
                ),
            ToggleSwitch::new()
                .header("Feature status")
                .is_on(self.enabled)
                .on_toggled(context.callback(Message::SetEnabled)),
            TextBlock::new().text(format!("Progress: {:.0}%", self.volume)),
            Slider::new()
                .minimum(0.0)
                .maximum(100.0)
                .step_frequency(1.0)
                .value(self.volume)
                .on_value_changed(context.callback(Message::VolumeChanged)),
            ProgressBar::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(self.volume),
        ))
    }

    fn layout_page(&self) -> StackPanel {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("Layout").font_size(24.0),
            TextBlock::new().text("Stack panels compose native controls in order."),
            StackPanel::new().spacing(8.0).children((
                TextBlock::new().text("First group"),
                TextBlock::new().text("Second group"),
                TextBlock::new().text("Third group"),
            )),
            StackPanel::new().spacing(8.0).children((
                Button::new().content("Primary action"),
                Button::new().content("Secondary action"),
            )),
            TextBlock::new().text(format!("The shared counter is {}.", self.count)),
        ))
    }

    fn about_page(&self) -> StackPanel {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("About this gallery").font_size(24.0),
            TextBlock::new()
                .text("The sample uses typed component messages to update native WinUI controls."),
            TextBlock::new().text("Runtime: Windows Reactor from the local windows-rs reference."),
            TextBlock::new().text("Packaging is intentionally left to winapp."),
        ))
    }
}

fn main() {
    App::run_component::<Gallery>(()).expect("failed to start the demo gallery");
}
