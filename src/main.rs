pub mod input_file_to_vector;
pub mod find_shared_letter_priority;
pub mod find_group_shared_letter;

fn main() {
    let rucksack_vector: Vec<String> = input_file_to_vector::text_to_vec();
    let mut iterator: usize = 0;
    let mut total_priority: u64 = 0;

    for _number in rucksack_vector.iter() {
        let line_length: usize = rucksack_vector[iterator].len();

        let mut first_half: String = "".to_string();
        let mut second_half: String = "".to_string();

        let mut character_vector: Vec<char> = rucksack_vector[iterator].chars().collect();
        let mut spliterator: usize = 0;

        for characters in character_vector {
            if spliterator < line_length / 2 {
                first_half.push(characters);
            } else {
                second_half.push(characters);
            }

            spliterator+= 1;
        }
        
        character_vector = first_half.chars().collect();

        for chars in character_vector {
            if second_half.contains(chars) {
                let shared_letter: char = chars;
                total_priority+= find_shared_letter_priority::find_priority(shared_letter);
                break;
            }
        }
        iterator+= 1;
    }

    println!("Total priority is: {}", total_priority);

    iterator = 0;
    let mut priority_of_badges: u64 = 0;

    while iterator < rucksack_vector.len() {
        let first_line_char_vector: Vec<char> = rucksack_vector[iterator].chars().collect();
        iterator+= 1;
        let second_line_char_vector: Vec<char> = rucksack_vector[iterator].chars().collect();
        iterator+= 1;
        let third_line_char_vector: Vec<char> = rucksack_vector[iterator].chars().collect();
        iterator+= 1;

        let group_shared_letter: char = find_group_shared_letter::find_group_shared_letter(first_line_char_vector, second_line_char_vector, third_line_char_vector);
        priority_of_badges+= find_shared_letter_priority::find_priority(group_shared_letter);
    }

    println!("Total priority of badges is: {}", priority_of_badges);
}
