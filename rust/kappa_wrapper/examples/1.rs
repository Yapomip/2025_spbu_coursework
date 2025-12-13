use kappa_wrapper::add;
use kappa_wrapper::test_call;
use kappa_wrapper::hello;

fn main() {
    let c = add(2, 2);
    println!("{c}");
    test_call(c as f32);
    unsafe { hello(); }
}
