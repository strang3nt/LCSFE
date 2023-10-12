/// Receives as input a string containing the
/// content of a file, returns as output the
/// vector of substrings separated by a newline
/// character. Thus an element of a basis
/// can be any kind of string.
pub fn basis_parser(s: String) -> Vec<String> {
    s.lines().map(|i| i.trim().to_string()).collect()
}
