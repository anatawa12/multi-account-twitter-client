use yew::prelude::*;
use super::html;

#[derive(Properties, PartialEq)]
pub struct HStackProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub struct HStack();

impl Component for HStack {
    type Message = ();
    type Properties = HStackProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("matc-hstack", ctx.props().class.clone())}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct VStackProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub struct VStack();

impl Component for VStack {
    type Message = ();
    type Properties = HStackProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("matc-vstack", ctx.props().class.clone())}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
