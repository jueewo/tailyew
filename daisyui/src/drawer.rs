use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub children: Children,
    #[prop_or_default]
    pub is_mobile: bool,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    // let test = vec!["drawer-mobile"];
    html! {
        <div class={classes!(
            "drawer",
            props.is_mobile.then(|| "drawer-mobile")
        )}>
        { for props.children.iter() }
        </div>

    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerContentProps {
    pub children: Children,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProps) -> Html {
    html! {
        <div class = {classes!("drawer-content")}>
        { for props.children.iter() }
        </div>

    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerSideProps {
    pub children: Children,
}

#[function_component(DrawerSide)]
pub fn drawer_side(props: &DrawerSideProps) -> Html {
    html! {
        <div class = {classes!("drawer-side")}>
        { for props.children.iter() }
        </div>

    }
}
