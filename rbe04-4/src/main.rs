fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50; // cannot assign twice to immutable variable `_mutable_integer`
    }

    _mutable_integer = 3;
}
