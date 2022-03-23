// // 填空并修复错误
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(__, __, __);
//     check_color(v);
// }
//
// fn check_color(p: Color) {
//     let (x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(__, 255);
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Color(0, 127, 255);
    check_color(v);
}

fn check_color(p: Color) {
    let Color(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}
