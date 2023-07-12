pub fn about() {
    println!("KolorWheel.rs {}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_nothing() {
        about();
        assert_eq!(2, 2);
    }
}
