pub fn formatvec_i32(vec: &Vec<i32>) -> String {
    if vec.is_empty() {
        return "".to_string(); // Handle empty vector
    }

    if vec.len() > 1 {
        let formatted: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
        formatted.join(", ")
    } else {
        vec[0].to_string()
    }
}
