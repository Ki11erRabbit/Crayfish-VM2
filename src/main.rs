use malachite::Natural;

pub mod value;
pub mod instruction;
pub mod stack;
pub mod program;
pub mod machine;

fn main() {
    let number: u64 = 77777777777777777;
    println!("{}", number);
    println!("{:?}", number.to_le_bytes());
    let natural: Natural = number.into();
    println!("{}", natural);
    println!("{:?}", natural);
}
