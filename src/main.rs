mod components;

use dioxus::prelude::{*, dioxus_elements::style};

use crate::components::button::button::Button;

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [ include_str!("./assets/antd.css") ] }
        style { [" button {margin-left: 8px; margin-right: 8px; margin-left: 8px; margin-right: 8px;}","
         section {border-style: outset; width: 50%;margin-top: 1px;}"]}
        div {
            style {["ant-col ant-col-12 code-boxes-col-2-1}"]}
        h1 { "代码演示"}
        section {
            class : "code-box-demo",
           
            Button { btype: "primary" ,text: "Primary Button"}
            Button { btype: "dashed" ,text: "Dashed Button"}
            br{}
            Button { btype: "text" ,text: "Text Button"}
            Button { btype: "link" ,text: "Link Button"}
            p {"按钮类型"}
            p { " 按钮有五种类型：主按钮、次按钮、虚线按钮、文本按钮和链接按钮。主按钮在同一个操作区域最多出现一次。"}
        }



        section {
            class : "code-box-demo",
            Button { btype: "primary" ,text: "Primary", size: "large"}
            Button { btype: "default" ,text: "Default"}
            Button { btype: "dashed" ,text: "Dashed", size: "small"}
        }
        section {
            class : "code-box-meta markdown",
            div { class : "code-box-description",
            div { 
                p { "按钮有大、中、小三种尺寸。"}
                p { "通过设置 size 为 large small 分别把按钮设为大、小尺寸。若不设置size 则尺寸为中" }
                }
                }
        }

    }
    ))
}
