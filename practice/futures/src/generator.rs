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
    Enter(i32),
    Yield1(i32),
    Exit,
}

impl GeneratorA {
    fn start(a1: i32) -> Self {
        GeneratorA::Enter(a1)
    }
}

impl Generator for GeneratorA {
    type Yield = i32;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        // match *self {
        match std::mem::replace(self, GeneratorA::Exit) {
            GeneratorA::Enter(a1) => {
                println!("Hello");
                let a = a1 * 2;
                *self = GeneratorA::Yield1(a);
                GeneratorState::Yielded(a)
            }
            GeneratorA::Yield1(_) => {
                println!("world");
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
        let mut gen = GeneratorA::start(4);
        if let GeneratorState::Yielded(n) = gen.resume() {
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
    //     let a: i32 = 4;
    //     let mut gen = move || {
    //         println!("Hello");
    //         yield a * 2;
    //         println!("world!");
    //     };
    //
    //     if let GeneratorState::Yielded(n) = gen.resume() {
    //         println!("Got value {}", n);
    //     }
    //
    //     if let GeneratorState::Complete(()) = gen.resume() {
    //         ()
    //     };
    // }
}