use std::rc::Weak;

use crate::declaration::{Node, View};
use crate::ir::{MountedNode, NodeKind};
use crate::native::{Runtime, dispatch};
use crate::xaml::{
    Button, CheckBox, IButtonBase, IContentControl, IPanel, IRangeBase, ISlider, IStackPanel,
    IStackPanel4, ITextBlock, ITextBox, ITextBox2, IToggleButton, IToggleSwitch,
    Orientation as XamlOrientation, ProgressBar, PropertyValue, RangeBaseValueChangedEventArgs,
    Slider, StackPanel, TextBlock, TextBox, ToggleSwitch, UIElement,
};
use crate::{Component, Result};
use windows_core::Interface;

pub(crate) fn render<C: Component>(
    runtime: &std::rc::Rc<std::cell::RefCell<Runtime<C>>>,
) -> Result<()> {
    let weak = std::rc::Rc::downgrade(runtime);
    let (view, old_root, window) = {
        let mut runtime = runtime.borrow_mut();
        let mut context = crate::component::ViewContext::new({
            let weak = weak.clone();
            move |message| dispatch(&weak, message)
        });
        let view = runtime.component.view(&runtime.input, &mut context);
        (view, runtime.root.take(), runtime.window.clone())
    };

    let mounted = reconcile_node(view, old_root, &weak)?;
    window.set_content(&mounted.element)?;
    runtime.borrow_mut().root = Some(mounted);
    Ok(())
}

fn reconcile_node<C: Component>(
    view: View,
    old: Option<MountedNode>,
    runtime: &Weak<std::cell::RefCell<Runtime<C>>>,
) -> Result<MountedNode> {
    let kind = node_kind(view.node());
    let mut mounted = match old {
        Some(old) if old.kind == kind => old,
        _ => MountedNode::new(kind, create_element(view.node())?),
    };
    mounted.event_revokers.clear();

    match view.0 {
        Node::TextBlock { text, font_size } => {
            let text_block: TextBlock = mounted.element.cast()?;
            let text_block: ITextBlock = text_block.cast()?;
            text_block.SetText(&text)?;
            text_block.SetFontSize(font_size)?;
            mounted.children.clear();
        }
        Node::Button { content, on_click } => {
            let button: Button = mounted.element.cast()?;
            let content_value = PropertyValue::CreateString(&content)?;
            let content_control: IContentControl = button.cast()?;
            content_control.SetContent(&content_value)?;
            if let Some(callback) = on_click {
                let callback = callback.clone();
                let revoker = button
                    .cast::<IButtonBase>()?
                    .Click(move |_, _| callback.invoke(()))?;
                mounted.event_revokers.push(revoker);
            }
            mounted.children.clear();
        }
        Node::StackPanel {
            orientation,
            spacing,
            children,
        } => {
            let panel: StackPanel = mounted.element.cast()?;
            panel
                .cast::<IStackPanel>()?
                .SetOrientation(match orientation {
                    crate::controls::Orientation::Vertical => XamlOrientation::Vertical,
                    crate::controls::Orientation::Horizontal => XamlOrientation::Horizontal,
                })?;
            panel.cast::<IStackPanel4>()?.SetSpacing(spacing)?;
            let children_collection = panel.cast::<IPanel>()?.Children()?;
            children_collection.Clear()?;

            let mut old_children = std::mem::take(&mut mounted.children).into_iter();
            let mut reconciled = Vec::with_capacity(children.len());
            for child in children {
                let child = reconcile_node(child, old_children.next(), runtime)?;
                children_collection.Append(&child.element)?;
                reconciled.push(child);
            }
            mounted.children = reconciled;
        }
        Node::TextBox {
            text,
            placeholder,
            on_text_changed,
        } => {
            let text_box: TextBox = mounted.element.cast()?;
            let text_box_interface: ITextBox = text_box.cast()?;
            if text_box_interface.Text()? != text {
                text_box_interface.SetText(&text)?;
            }
            text_box
                .cast::<ITextBox2>()?
                .SetPlaceholderText(&placeholder)?;
            if let Some(callback) = on_text_changed {
                let revoker = text_box_interface.TextChanged(move |sender, _| {
                    let text = sender
                        .as_ref()
                        .and_then(|sender| sender.cast::<TextBox>().ok())
                        .and_then(|sender| sender.cast::<ITextBox>().ok())
                        .and_then(|sender| sender.Text().ok())
                        .unwrap_or_default();
                    callback.invoke(text);
                })?;
                mounted.event_revokers.push(revoker);
            }
            mounted.children.clear();
        }
        Node::CheckBox {
            content,
            is_checked,
            on_checked_changed,
        } => {
            let check_box: CheckBox = mounted.element.cast()?;
            let content_value = PropertyValue::CreateString(&content)?;
            check_box
                .cast::<IContentControl>()?
                .SetContent(&content_value)?;
            let toggle: IToggleButton = check_box.cast()?;
            toggle.SetIsChecked(Some(is_checked))?;
            if let Some(callback) = on_checked_changed {
                let checked = callback.clone();
                mounted
                    .event_revokers
                    .push(toggle.Checked(move |_, _| checked.invoke(true))?);
                mounted
                    .event_revokers
                    .push(toggle.Unchecked(move |_, _| callback.invoke(false))?);
            }
            mounted.children.clear();
        }
        Node::ToggleSwitch {
            header,
            is_on,
            on_toggled,
        } => {
            let toggle: ToggleSwitch = mounted.element.cast()?;
            let toggle_interface: IToggleSwitch = toggle.cast()?;
            toggle_interface.SetHeader(&PropertyValue::CreateString(&header)?)?;
            toggle_interface.SetIsOn(is_on)?;
            if let Some(callback) = on_toggled {
                let source = toggle.clone();
                mounted
                    .event_revokers
                    .push(toggle_interface.Toggled(move |_, _| {
                        callback.invoke(
                            source
                                .cast::<IToggleSwitch>()
                                .and_then(|source| source.IsOn())
                                .unwrap_or(false),
                        );
                    })?);
            }
            mounted.children.clear();
        }
        Node::Slider {
            minimum,
            maximum,
            step_frequency,
            value,
            on_value_changed,
        } => {
            let slider: Slider = mounted.element.cast()?;
            let range: IRangeBase = slider.cast()?;
            range.SetMinimum(minimum)?;
            range.SetMaximum(maximum)?;
            range.SetValue(value)?;
            slider.cast::<ISlider>()?.SetStepFrequency(step_frequency)?;
            if let Some(callback) = on_value_changed {
                mounted.event_revokers.push(range.ValueChanged(
                    move |_, args: windows_core::Ref<RangeBaseValueChangedEventArgs>| {
                        if let Some(value) = args.as_ref().and_then(|args| args.NewValue().ok()) {
                            callback.invoke(value);
                        }
                    },
                )?);
            }
            mounted.children.clear();
        }
        Node::ProgressBar {
            minimum,
            maximum,
            value,
        } => {
            let progress: ProgressBar = mounted.element.cast()?;
            let range: IRangeBase = progress.cast()?;
            range.SetMinimum(minimum)?;
            range.SetMaximum(maximum)?;
            range.SetValue(value)?;
            mounted.children.clear();
        }
    }

    Ok(mounted)
}

fn node_kind(node: &Node) -> NodeKind {
    match node {
        Node::TextBlock { .. } => NodeKind::TextBlock,
        Node::Button { .. } => NodeKind::Button,
        Node::StackPanel { .. } => NodeKind::StackPanel,
        Node::TextBox { .. } => NodeKind::TextBox,
        Node::CheckBox { .. } => NodeKind::CheckBox,
        Node::ToggleSwitch { .. } => NodeKind::ToggleSwitch,
        Node::Slider { .. } => NodeKind::Slider,
        Node::ProgressBar { .. } => NodeKind::ProgressBar,
    }
}

fn create_element(node: &Node) -> Result<UIElement> {
    match node {
        Node::TextBlock { .. } => Ok(TextBlock::new()?.cast()?),
        Node::Button { .. } => Ok(Button::new()?.cast()?),
        Node::StackPanel { .. } => Ok(StackPanel::new()?.cast()?),
        Node::TextBox { .. } => Ok(TextBox::new()?.cast()?),
        Node::CheckBox { .. } => Ok(CheckBox::new()?.cast()?),
        Node::ToggleSwitch { .. } => Ok(ToggleSwitch::new()?.cast()?),
        Node::Slider { .. } => Ok(Slider::new()?.cast()?),
        Node::ProgressBar { .. } => Ok(ProgressBar::new()?.cast()?),
    }
}
