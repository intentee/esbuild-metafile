pub fn paths(paths: &[&str]) -> Vec<String> {
    paths.iter().map(ToString::to_string).collect()
}
