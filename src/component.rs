use crate::style::Style;
// use crate::tree::Tree;

/*
  Elements:
  - View
  - Canvas
  - Image (canvas)
  - Component
 */

pub trait Node {}

pub trait Element: Node {
	fn get_id(&self) -> String;
	fn get_focus(&self) -> String;
	fn get_hover(&self) -> String;
	fn get_style(&self) -> Style;

	fn set_id(&mut self, id: String);
	fn set_focus(&mut self, flag: bool);
	fn set_hover(&mut self, flag: bool);
	fn set_style(&mut self, style: String);
}

impl Node for String {}

// pub struct View {
//     id: String,
//     style: Style,
// }

// impl Node for View {
//     fn set_id(&self, id: String) {
//         self.id = id;
//     }

//     fn get_id(&self) -> String {
//         self.id
//     }

//     fn set_style(&self, style: String) {
//         self.style = style;
//     }

//     fn get_style(&self) -> Style {
//         self.style
//     }
// }

// #[derive(Debug, Default, Clone)]
// pub struct Component {
//     id: String,
//     style: Style,
//     tree: Option<Tree>,
// }

// impl Component {
//     fn styles() -> Vec<Style> {
//         Vec::new()
//     }

//     fn properties() -> Vec<String> {
//         Vec::new()
//     }

//     fn set_id(&self, id: String) {
//         self.id = id;
//     }

//     fn get_id(&self) -> String {
//         self.id
//     }

//     fn set_style(&self, style: String) {
//         self.style = style;
//     }

//     fn get_style(&self) -> Style {
//         self.style
//     }

//     fn should_update(&self, changed: Vec<String>) -> bool {
//         return true
//     }

//     fn process(&self) {
//         let changed = Vec::new();

//         if self.should_update(changed) {
//             self.will_update(changed);
//             let tree = self.render();

//             self.did_update(changed);
//         }
//     }

//     pub fn will_mount(&self) {}

//     pub fn did_mount(&self) {}

//     pub fn will_unmount(&self) {}

//     pub fn did_unmount(&self) {}

//     pub fn will_update(&self, changed: Vec<String>) {}

//     pub fn did_update(&self, changed: Vec<String>) {}

//     pub fn render(&self) -> Tree {
//         return Tree::default()
//     }
// }
