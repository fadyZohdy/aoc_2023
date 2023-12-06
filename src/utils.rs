pub fn get_input_file_name(path: &str) -> String {
    path.split("::").last().unwrap().trim_end().to_string() + "_input.txt"
}