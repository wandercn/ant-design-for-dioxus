use dioxus::prelude::*;

#[derive(Props, Default)]
pub struct ButtonProps<'a> {
    pub btype: &'a str,
    pub text: &'a str,
    #[props(default = "")]
    pub size: &'a str,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let btype = "ant-btn-".to_string() + cx.props.btype;
    let text = cx.props.text;
    let size = match cx.props.size {
        "large" => "ant-btn-lg",
        "small" => "ant-btn-sm",
        _ => "",
    };
    cx.render(rsx! {
        button {
            r#type: "button",
            class: "ant-btn {btype} {size}",
            size : "{size}",
            span {
                "{text}"
            }
        }
    })
}
