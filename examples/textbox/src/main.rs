mod theme;

use crate::theme::*;

use zensen::{
    component::Element,
    style::{
        Scalar,
        BorderKind,
        BackgroundFill,
        Style,
        StyleBuilder,
    },
};

// fn style_root(c: Textbox) -> StyleBuilder {
//     StyleBuilder::default()
//         .border(Scalar::Px(1), BorderKind::Solid, COLOR_GREY_1)
//         .border_radius(Scalar::Px(3))
//         .background_fill(BackgroundFill::Color(COLOR_WHITE))
// }

// fn style_root_focused(c: Textbox) -> StyleBuilder {
//     if c.focused {
//         StyleBuilder::default().border_width(Scalar::Px(2))
//     } else {
//         StyleBuilder::default()
//     }
// }

// fn style_root_invalid(c: Textbox) -> StyleBuilder {
//     if c.focused && !c.invalid && !c.disabled {
//         StyleBuilder::default().border_color(COLOR_HIGHLIGHT)
//     } else {
//         StyleBuilder::default()
//     }
// }

// fn style_root_disabled(c: Textbox) -> StyleBuilder {
//     if c.disabled {
//         StyleBuilder::default().border_color(COLOR_GREY_3)
//     } else {
//         StyleBuilder::default()
//     }
// }

// fn style_container(c: Textbox) -> StyleBuilder {
//     StyleBuilder::default().spacing_hv(Scalar::Px(0), Scalar::Px(c.indent))
// }

// fn style_focused(c: Textbox) -> StyleBuilder {
//     if c.focused {
//         StyleBuilder::default().spacing_hv(Scalar::Px(c.indent - 1), Scalar::Zero)
//     } else {
//         StyleBuilder::default()
//     }
// }

// fn style_container_hover(c: Textbox) -> StyleBuilder {
//     if c.hover && !c.invalid && !c.disabled {
//         StyleBuilder::default().border_color(COLOR_BLACK)
//     } else {
//         StyleBuilder::default()
//     }
// }

// fn style_container_invalid(c: Textbox) -> StyleBuilder {
//     if c.invalid && !c.disabled {
//         StyleBuilder::default().border_color(COLOR_ERROR)
//     } else {
//         StyleBuilder::default()
//     }
// }

// #[derive(Element)]
pub struct Textbox {
    // #[prop(pub)]
    focused: bool,
    // #[prop(pub)]
    invalid: bool,
    // #[prop(pub)]
    disabled: bool,
    // #[prop()]
    indent: i32,
}

// impl Component for Textbox {
//     fn new() -> Textbox {
//         Textbox {
//             focused: false,
//             invalid: false,
//             disabled: false,
//             indent: 12,
//         }
//     }

//     fn render(&self) -> Template {
//         template![
//             <View styles:[container]>
//                 <Slot/>
//             </View>
//         ]
//     }
// }

fn main() {
    println!("rendering textbox...");
    println!("sizeof Style: {}", std::mem::size_of::<Style>());
    println!("sizeof StyleBuilder: {}", std::mem::size_of::<StyleBuilder>());
}
