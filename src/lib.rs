pub use assets::AssetImage;

mod screen;
mod image;
mod color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let left = 2 + 2;
        let right = 4;
        assert_eq!(left, right);
    }
}
