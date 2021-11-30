enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

trait Generator {
    type Yield;
    type Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA {
    Enter,
    Yield1 {
        to_borrow: String,
        borrowed: *const String, // NB! This is now a raw pointer!
    },
    Exit,
}

impl GeneratorA {
    fn start() -> Self {
        GeneratorA::Enter
    }
}

impl Generator for GeneratorA {
    type Yield = usize;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        match self {
            GeneratorA::Enter => {
                let to_borrow = String::from("Hello");
                let borrowed = &to_borrow;
                let res = borrowed.len();

                *self = GeneratorA::Yield1 { to_borrow, borrowed: std::ptr::null() };

                if let GeneratorA::Yield1 { to_borrow, borrowed } = self {
                    *borrowed = to_borrow;
                }

                GeneratorState::Yielded(res)
            }
            GeneratorA::Yield1 { to_borrow, borrowed } => {
                let borrowed: &String = unsafe { &**borrowed };
                println!("{} world", borrowed);
                *self = GeneratorA::Exit;
                GeneratorState::Complete(())
            }
            GeneratorA::Exit => panic!("Can't advance an exited generator!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let mut gen = GeneratorA::start();
        let mut gen2 = GeneratorA::start();

        if let GeneratorState::Yielded(n) = gen.resume() {
            println!("Got value {}", n);
        }

        std::mem::swap(&mut gen, &mut gen2); // <--- Big problem!

        if let GeneratorState::Yielded(n) = gen2.resume() {
            println!("Got value {}", n);
        }

        if let GeneratorState::Complete(()) = gen.resume() {
            ()
        };
    }

    // #[test]
    // fn it_works() {
    //     #![feature(generators, generator_trait)]
    //     use std::ops::{Generator, GeneratorState};
    //     let mut generator = move || {
    //         let to_borrow = String::from("Hello");
    //         let borrowed = &to_borrow;
    //         yield borrowed.len();
    //         println!("{} world!", borrowed);
    //     };
    //
    //     while let Some(val) = generator.next() {
    //         println!("{}", val);
    //     }
    // }
}