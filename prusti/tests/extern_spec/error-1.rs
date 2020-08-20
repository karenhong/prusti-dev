#![feature(register_tool)]
#![register_tool(prusti)]

extern crate prusti_contracts;
use prusti_contracts::*;

#[extern_spec]
impl<T> std::vec::Vec::<T> {
    #[ensures(self.len() == old(self.len()) + 1)]
    fn push(&mut self);

    #[ensures(self.len() == 0)]
    fn clears(&mut self);
}

fn main() {}