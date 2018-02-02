mod isKanji;
mod utils::isCharKanji;

#[cfg(test)]
mod tests {
    use isKanji;
    #[test]
    fn it_works() {
        assert_eq!(isKanji('刀'), true);
    }
}
