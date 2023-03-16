use unwrap_enum::EnumIs;

#[derive(EnumIs)]
enum E<T> {
    A,
    B { a: T },
    C(T),
}
