#![allow(dead_code)]

use unwrap_enum::{EnumAs, EnumIs};

#[derive(EnumIs, EnumAs)]
enum Shape {
    Rect { height: usize, length: usize },
    Line(usize),
    Dot,
}

#[derive(EnumIs, EnumAs)]
enum Event<'a> {
    Clicked(&'a Shape),
    Moved(&'a Shape, (usize, usize)),
}

#[test]
fn test_enum_is() {
    let rect = Shape::Rect {
        length: 1,
        height: 1,
    };
    let line = Shape::Line(1);
    let dot = Shape::Dot;
    assert!(rect.is_rect());
    assert!(line.is_line());
    assert!(dot.is_dot());
}

#[test]
fn test_enum_as() {
    let rect = Shape::Rect {
        length: 1,
        height: 1,
    };
    let line = Shape::Line(1);
    assert!(matches!(rect.as_rect(), Some((1, 1))));
    assert!(matches!(line.as_line(), Some(1)));
}
