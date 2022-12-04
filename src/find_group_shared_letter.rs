pub fn find_group_shared_letter(first_line: Vec<char>, second_line: Vec<char>, third_line: Vec<char>) -> char {
    let mut shared_letter: char = '\0';
    
    for letter in first_line {
        if second_line.contains(&letter) {
            if third_line.contains(&letter) {
                shared_letter = letter;
                break;
            }
        }
    }

    return shared_letter
}