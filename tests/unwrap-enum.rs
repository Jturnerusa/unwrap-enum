#![allow(dead_code)]

use unwrap_enum::{EnumAs, EnumAsMut, EnumInto, EnumIs};

#[derive(Debug, Clone, Copy, EnumIs, EnumAs, EnumAsMut, EnumInto)]
enum Shape {
    Rect { height: usize, length: usize },
    Line(usize),
    Dot,
}

// This isn't used in tests below, it's used to make sure the macros don't conflict with
// the generic lifetimes in the definition.

#[derive(Debug, Clone, Copy, EnumIs, EnumAs, EnumAsMut, EnumInto)]
enum Event<'a> {
    Clicked(&'a Shape),
    Moved(&'a Shape, (usize, usize)),
}

#[test]
fn unwrap_enum() {
    let mut rect = Shape::Rect {
        height: 682,
        length: 671,
    };
    let line = Shape::Line(285);
    let dot = Shape::Dot;
    assert!(rect.is_rect() && line.is_line() && dot.is_dot());
    *rect.as_mut_rect().unwrap().0 = 907;
    assert!(matches!(rect.as_rect(), Some((907, 671))));
    assert!(matches!(line.as_line(), Some(285)));
}
