#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_parse_basic() {
        let input = "---\ntitle: Hello\n---\n\nWorld\n";
        let result = parse(input).expect("Should parse");
        assert_eq!(result.frontmatter.title, "Hello");
        assert_eq!(result.blocks.len(), 1);
    }
}
