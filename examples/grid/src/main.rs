use zensen::style::{SizePolicy, StyleBuilder};

fn style_root() -> StyleBuilder {
    StyleBuilder::default()
        .layout_rows(SizePolicy::repeat(3, SizePolicy::Fr(1)))
}

// #[derive(Component)]
// pub struct Sidebar {
//     styles! {
//         root {
//             layout: {
//                 template: Layout::Template::Rows(
//                     SizePolicy::repeat(3, ::Fr(1)),
//                 ),
//             },
//         }

//         item_left {
//             frame: {
//                 align_self_h: Align::Start,
//             },
//         }

//         item_right {
//             frame: {
//                 align_self_h: Align::End,
//             },
//         }

//         item_top {
//             frame: {
//                 align_self_v: Align::Start,
//             },
//         }

//         item_bottom {
//             frame: {
//                 align_self_v: Align::End,
//             },
//         }
//     }

//     fn render(&self) -> Markup {
//         return markup! {
//             <View styles: [item_top, item_left]></View>
//             <View styles: [item_top]></View>
//             <View styles: [item_top, item_right]></View>

//             <View styles: [item_left]></View>
//             <View></View>
//             <View styles: [item_right]></View>

//             <View styles: [item_bottom, item_left]></View>
//             <View styles: [item_bottom]></View>
//             <View styles: [item_bottom, item_right]></View>
//         }
//     }
// }

struct Login {
}

fn test() {
    {}
}

fn main() {
    println!("Hello world. We're building a grid.");
}
