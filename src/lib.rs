extern crate func;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    #[test]
    fn test_constant() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
