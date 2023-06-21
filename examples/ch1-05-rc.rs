use std::rc::Rc;

fn main() {
    let a = Rc::new([1, 2, 3]);
    #[allow(clippy::redundant_clone)]
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
}
