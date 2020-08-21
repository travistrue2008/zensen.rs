# zensen.rs

A browser-like UI framework for Rust

## Element Hierarchy

The user interface is built from a tree-like structure composed of nodes. Each node can be either a text node or a component. UI elements are purely composed component-like elements. Each element has these-level properties:
- ID: a unique identifier within the scope of the current component
- Styles: a set of visual styles. Styles can be associated with expressions to determine whether or not they're applied.
- Each 

## Element Types
- View
- Canvas
- Component

### Access

- Components encapsulate their IDS and style selectors
- Components have direct access to all of their nodes within their own local DOM
- Components do not have direct access to elements in their child components' local DOM
- DOM references only act on the node's ID field

### DOM Ref Queries

- `exact()`
- `starts_with()`
- `ends_with()`
- `regex()`

## Defining Styles

- Components own their styles
- Styles have full access to renderable props
- Selectors
  - Must be assigned to elements
  - Rendable props can be used in expressions to conditionally activate selectors

## Base Props

- `id`: Used for selecting an element. An error is thrown if not unique.
- `styles`: Array of style selectors
- `tab`: Enum for tab order with the following variants: `None`, `Enabled`, `Index(value)`
- `focus`: When the element is in focus state
- `hover`: When the element is in hovering state

## Lifecycle Methods

- `will_mount()`
- `did_mount()`
- `will_unmount()`
- `did_unmount()`
- `will_update(changed)`
- `did_update(changed)`
- `should_update(changed)`
- `render()`

## Templating Language

It looks very HTML-like in nature:

```tpl
<test
  id: ""
  styles: []
>I am a test</test>
```

## Component Trait

```rust
pub trait Component {
  fn new() -> Self;
  fn styles() -> Vec<Style>;
  fn properties() -> Vec<String>;

  fn will_mount();
  fn did_mount();
  fn will_unmount();
  fn did_unmount();
  fn will_update(changed);
  fn did_update(changed);
  fn should_update(changed);
  fn render();
}
```

## Syntax

```rust
component! {
}
```

## Roadmap

- Rendering engine
- Layouts
- Text rendering
- Asset management
- SVG
- Drag and Drop
- Canvas
- Event propagation
- SEO
