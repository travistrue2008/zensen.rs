use zensen::style::{Align, SizePolicy, StyleBuilder};

fn style_root(c: Grid) -> StyleBuilder {
    StyleBuilder::default()
        .layout_rows(SizePolicy::repeat(3, SizePolicy::Fr(1)))
}

fn style_item_left(c: Grid) -> StyleBuilder {
    StyleBuilder::default().align_self_h(Align::Start)
}

fn style_item_right(c: Grid) -> StyleBuilder {
    StyleBuilder::default().align_self_h(Align::End)
}

fn style_item_top(c: Grid) -> StyleBuilder {
    StyleBuilder::default().align_self_v(Align::Start)
}

fn style_item_bottom(c: Grid) -> StyleBuilder {
    StyleBuilder::default().align_self_v(Align::End)
}

// #[derive(Component)]
pub struct Grid {
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
}

fn main() {
    println!("Hello world. We're building a grid.");
}
