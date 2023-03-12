use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub children: Children,
    // pub data-tip: String,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    html! {
        <div class={classes!("tooltip")} data-tip="hello">
            {for props.children.iter()}
        </div>
    }
}
