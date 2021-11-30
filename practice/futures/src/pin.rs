use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    // fn init(self: Pin<&mut Self>) {
    //     // let self_ref: *const String = &self.a;
    //     // self.b = self_ref;
    //
    //     let self_ptr: *const String = &self.a;
    //     let this = unsafe { self.get_unchecked_mut() };
    //     this.b = self_ptr;
    // }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test1 = Test::new("test1");
        // let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        // Test::init(test1.as_mut());

        let mut test2 = Test::new("test2");
        // let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        // Test::init(test2.as_mut());

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());

        // std::mem::swap(test1.get_mut(), test2.get_mut());

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    }
}