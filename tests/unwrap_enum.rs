use unwrap_enum::EnumIs;

macro_rules! is {
    ($a:expr, $($method:ident),+) => {
        $($a.$method() &&)+ true
    };
}

#[derive(Debug, Clone)]
struct Key;

#[allow(dead_code)]
#[derive(Debug, Clone, EnumIs)]
enum Event<'a, T = ()> {
    KeyPress(Key),
    MouseMove(u64, u64),
    Message(&'a str),
    Other(T),
    Quit,
}

#[test]
fn unwrap_enum() {
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
