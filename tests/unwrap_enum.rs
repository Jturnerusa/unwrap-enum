use unwrap_enum::{EnumAs, EnumIs};

macro_rules! is {
    ($a:expr, $($method:ident),+) => {
        $($a.$method() &&)+ true
    };
}

#[derive(Debug, Clone)]
struct Key;

#[allow(dead_code)]
#[derive(Debug, Clone, EnumIs, EnumAs)]
enum Event<'a, T = ()> {
    KeyPress(Key),
    MouseMove(u64, u64),
    Message(&'a str),
    Other(T),
    Quit,
}

#[test]
fn enum_is() {
    assert!({
        let event = Event::KeyPress::<()>(Key);
        event.is_keypress() && !is!(event, is_mousemove, is_message, is_other, is_quit)
    });
    assert!({
        let event = Event::MouseMove::<()>(10, 11);
        event.is_mousemove() && !is!(event, is_keypress, is_message, is_other, is_quit)
    });
    assert!({
        let event = Event::Message::<()>("ferris");
        event.is_message() && !is!(event, is_keypress, is_mousemove, is_other, is_quit)
    });
    assert!({
        let event = Event::Other(());
        event.is_other() && !is!(event, is_keypress, is_mousemove, is_message, is_quit)
    });
    assert!({
        let event = Event::Quit::<()>;
        event.is_quit() && !is!(event, is_keypress, is_message, is_mousemove, is_other)
    });
}

#[test]
fn enum_as() {
    assert!({
        let event = Event::KeyPress::<()>(Key);
        matches!(event.as_keypress(), Some(Key))
            && event.as_mousemove().is_none()
            && event.as_message().is_none()
            && event.as_other().is_none()
    });
    assert!({
        let event = Event::MouseMove::<()>(10, 11);
        matches!(event.as_mousemove(), Some((10, 11)))
            && event.as_keypress().is_none()
            && event.as_message().is_none()
            && event.as_other().is_none()
    });
    assert!({
        let event = Event::Message::<()>("ferris");
        matches!(event.as_message().copied(), Some("ferris"))
            && event.as_keypress().is_none()
            && event.as_mousemove().is_none()
            && event.as_other().is_none()
    });
    assert!({
        let event = Event::Other::<()>(());
        matches!(event.as_other(), Some(()))
            && event.as_keypress().is_none()
            && event.as_mousemove().is_none()
            && event.as_message().is_none()
    });
}
