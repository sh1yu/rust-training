trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

#[repr(C)]
struct FatPointer<'a> {
    data: &'a mut Data,
    vtable: *const usize,
}

struct Data {
    a: i32,
    b: i32,
}

fn add(s: &Data) -> i32 {
    s.a + s.b
}

fn sub(s: &Data) -> i32 {
    s.a - s.b
}

fn mul(s: &Data) -> i32 {
    s.a * s.b
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of, transmute};
    use super::*;

    #[test]
    fn it_works() {
        let mut data = Data { a: 3, b: 2 };
        let vtable = vec![0, size_of::<Data>(), align_of::<Data>(), add as usize, sub as usize, mul as usize];
        let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr() };
        let test = unsafe { transmute::<FatPointer, &dyn Test>(fat_pointer) };

        println!("Add: 3 + 2 = {}", test.add());
        println!("Sub: 3 - 2 = {}", test.sub());
        println!("Mul: 3 * 2 = {}", test.mul());
    }
}
