pub trait MutClosure {
    fn call<F: FnMut() -> () + Send + 'static>(_: F);
}
pub trait OnceClosure {
    fn call<F: FnOnce() -> () + Send + 'static>(_: F);
}

pub struct Closure;
impl Closure {
    pub fn call<F: FnOnce() -> () + Send + 'static>(f: F) {
        f();
    }
}
impl MutClosure for Closure {
    fn call<F: FnMut() -> () + Send + 'static>(mut f: F) {
        f();
    }
}
impl OnceClosure for Closure {
    fn call<F: FnOnce() -> () + Send + 'static>(f: F) {
        f();
    }
}