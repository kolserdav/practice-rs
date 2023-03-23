use std::any::Any;

trait Copy {
    fn copy(self) -> String;
    fn as_any(&self) -> &dyn Any;
}

trait Parse {
    fn parse(self) -> u8;
}

#[derive(Debug)]
struct S(String);

impl Copy for S {
    fn copy(self) -> String {
        self.0.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Parse for S {
    fn parse(self) -> u8 {
        self.0.parse::<u8>().unwrap()
    }
}

fn start<T>(t: T)
where
    T: Copy + Parse + std::fmt::Debug,
{
    println!("{:?}", t);
    println!("{}", t.parse());
}

pub fn impl_trait() {
    let st = S(String::from("8"));
    start(st);
}
