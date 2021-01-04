use crate::theme::{
    COLOR_BLACK,
    COLOR_WHITE,
    COLOR_ERROR,
    COLOR_HIGHLIGHT,
    COLOR_GREY_1,
    COLOR_GREY_3,
};

use zensen::style::Style;

fn style_root(c: Textbox) -> Style {
    Style {
        border: Border::new(1, theme::COLOR_GREY_1, BorderKind::Solid),
        background: Background::color(theme::COLOR_WHITE),
    }
}

fn style_container(c: Textbox) -> Style {
    Style {
        padding: Sides::make_xy(0, c.indent),
    }
}

fn style_container_hover(c: Textbox) -> Style {
    if c.hover && !c.invalid && !c.disabled {
        Style {
          border-color: ${CSS_COLOR_BLACK};
        }
    } else {
        Style::new()
    }
}

/*
  Element implements:
  - pub id(&self)
  - pub focus(&self)
  - pub hover(&self)
  - pub styles(&self)

  - pub set_id(&self, v: String)
  - pub set_focus(&self, v: Bool)
  - pub set_hover(&self, v: Bool)
  - pub set_styles(&self, v: Vec<Styles>)
 */

/*
  Root implements:
  - width(&self)
  - height(&self)
  - set_width(&self)
  - get_width(&self)
 */

#[derive(Element)]
pub struct Textbox {
    #[prop(pub)]
    focused: Bool,
    #[prop(pub)]
    invalid: Bool,
    #[prop(pub)]
    disabled: Bool,
    #[prop()]
    indent: u32,
}

impl Component for Textbox {
    fn new() -> Textbox {
        Textbox {
            focused: false,
            invalid: false,
            disabled: false,
            indent: 12,
        }
    }

    fn render(&self) -> Template {
        template![
            [View styles:[]]
                [Slot]
            [/View]
        ]
    }
}

// class Textbox extends LitElement {
//   static get styles () {
//     return [
//       css`
//         :host(:hover:not([invalid]):not([disabled])) {
//           border-color: ${CSS_COLOR_BLACK};
//         }

//         :host([invalid]:not([disabled])) {
//           border-color: ${CSS_COLOR_ERROR};
//         }

//         :host([focused]) {
//           border-width: 2px;
//         }

//         :host([focused]:not([invalid]):not([disabled])) {
//           border-color: ${CSS_COLOR_HIGHLIGHT};
//         }

//         :host([disabled]) {
//           border-color: ${CSS_COLOR_GREY_3};
//         }

//         .container {
//           display: flex;
//           width: 100%;
//           height: 100%;
//           padding: 0 var(--indent);
//         }

//         :host([focused]) .container {
//           padding: 0 calc(var(--indent) - 1px);
//         }
//       `,
//     ]
//   }

//   constructor () {
//     super()
//     this.initState()
//   }

//   initState () {
//     this.focused = false
//     this.invalid = false
//   }

//   render () {
//     return html`
//       <div class="container">
//         <slot></slot>
//       </div>
//     `
//   }
// }

// window.customElements.define('neb-textbox', Textbox)
