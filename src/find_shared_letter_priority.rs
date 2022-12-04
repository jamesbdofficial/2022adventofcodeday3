pub fn find_priority(shared_letter: char) -> u64 {
    // lowercase a - z have priorities 1 through 26.
    // uppercase A - Z have priorities 27 through 52.

    let uppercase: bool = shared_letter.is_uppercase();
    let alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut priority: usize = 0;
    
    if uppercase { // the letter is uppercase.
        let mut iterator: usize = 0;
        let lowercase_vector: Vec<char> = shared_letter.to_lowercase().collect();
        let lowercase_shared_letter = lowercase_vector[0];
        for letter in alphabet {
            if lowercase_shared_letter == letter {
                priority = iterator + 27;
                return priority as u64;
            }
            iterator+= 1;
        }
    } else { // the letter is not uppercase.
        let mut iterator: usize = 0;
        for letter in alphabet {
            if shared_letter == letter {
                priority = iterator + 1;
                return priority as u64;
            }
            iterator+= 1;
        }
    }

    return priority as u64;
}