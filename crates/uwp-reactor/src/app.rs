mod lifecycle;

pub(crate) use lifecycle::start;

use crate::Result;
use crate::component::Component;
use crate::declaration::View;
use crate::native::ApplicationRuntime;

#[derive(Clone, Copy, Debug, Default)]
pub struct App;

impl App {
    pub fn run(view: impl Into<View>) -> Result<()> {
        ApplicationRuntime::run_component::<StaticView>(view.into())
    }

    pub fn run_component<C: Component>(input: C::Input) -> Result<()> {
        ApplicationRuntime::run_component::<C>(input)
    }
}

struct StaticView;

impl Component for StaticView {
    type Input = View;
    type Message = ();

    fn create(_: &View, _: &crate::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &View, _: &mut crate::ViewContext<Self>) -> View {
        input.clone()
    }
}
