use std::marker::PhantomData;
use std::rc::Rc;

use crate::View;

#[derive(Clone)]
pub struct Callback<T> {
    handler: Rc<dyn Fn(T)>,
}

impl<T> Callback<T> {
    pub(crate) fn new(handler: impl Fn(T) + 'static) -> Self {
        Self {
            handler: Rc::new(handler),
        }
    }

    pub(crate) fn invoke(&self, value: T) {
        (self.handler)(value);
    }
}

pub struct ComponentContext<C: Component> {
    marker: PhantomData<fn() -> C>,
}

impl<C: Component> ComponentContext<C> {
    pub(crate) fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

pub struct ViewContext<C: Component> {
    send: Rc<dyn Fn(C::Message)>,
}

impl<C: Component> ViewContext<C> {
    pub(crate) fn new(send: impl Fn(C::Message) + 'static) -> Self {
        Self {
            send: Rc::new(send),
        }
    }

    pub fn callback<T: 'static>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        let send = Rc::clone(&self.send);
        Callback::new(move |value| send(map(value)))
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone + 'static,
    {
        let send = Rc::clone(&self.send);
        Callback::new(move |()| send(message.clone()))
    }

    pub fn window_frame(&mut self, title: impl Into<String>, content: impl Into<View>) -> View {
        use crate::controls::{Orientation, StackPanel, TextBlock};

        StackPanel::new()
            .orientation(Orientation::Vertical)
            .spacing(16.0)
            .children([
                TextBlock::new().text(title.into()).font_size(28.0).into(),
                content.into(),
            ])
            .into()
    }
}

pub trait Component: Sized + 'static {
    type Input: 'static;
    type Message: 'static;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self;

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View;
}
