// This is free and unencumbered software released into the public domain.

pub fn main() {
    for dev in egpu::list_enclosures().unwrap() {
        println!("{:#?}", dev);
    }
}
