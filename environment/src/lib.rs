#![deny(rust_2018_idioms)]

pub mod collision;
pub mod object;
pub mod traits;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
