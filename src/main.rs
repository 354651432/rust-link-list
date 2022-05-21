struct A;

fn f(_: &A) {}

fn main() {
    // let len = 20;
    // for i in 0..=len {
    //     println!("{i}")
    // }
    let a = A;
    f(&a);
    f(Box::new(&a).as_ref());
    // f(&[a]);
}
