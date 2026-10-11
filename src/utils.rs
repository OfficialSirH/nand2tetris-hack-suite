pub fn multiline_to_clean_vec(value: &str) -> Vec<String> {
    value
        .replace(" ", "")
        .split_terminator('\n')
        .filter_map(|str| (!str.is_empty()).then(|| str.to_owned()))
        .collect::<Vec<String>>()
}
