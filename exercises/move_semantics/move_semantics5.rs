// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x;  *y += 100;
    let z = &mut x;  // 这里y和z都是可变引用，所以不能同时存在

    *z += 1000;
    assert_eq!(x, 1200);
}
