pub type Closure = Box<dyn FnOnce() -> () + Send + 'static>;
pub type AdvClosure = Box<dyn Fn() -> () + Send + 'static>;

pub struct Clossures;
impl Clossures {
    pub fn safe_closure<F: FnOnce() -> () + Send + 'static>(f: F){
        f();
    }
    pub unsafe fn unsafe_closure<F: FnOnce() -> () + Send + 'static>(f: F){
        f();
    }
    pub fn uclosure<F: FnOnce() -> () + Send + 'static>(f: F){
        #[allow(unused_unsafe)]
        unsafe {
            f();
        };
    }
}