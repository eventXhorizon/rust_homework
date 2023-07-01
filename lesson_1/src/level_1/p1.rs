
// 循环打印 'a'-'Z' 之间的所有字符     a`_^]\[Z
pub fn print_a_z_1() {
    for c in ('A'..='z').rev() {
        if c <= 'a' && c >= 'Z' {
            print!("{}", c);
        }
    }
    println!();
}