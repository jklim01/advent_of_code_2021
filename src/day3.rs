use std::process;

type BinaryWord = Vec<bool>;

enum BitCriteria {
    Oxygen, Co2
}
impl BitCriteria {
    fn get_bit_to_match(&self, one_count: usize, num_of_words: usize) -> bool {
        match self {
            BitCriteria::Oxygen => return one_count >= (num_of_words+1)/2,
            BitCriteria::Co2 => return one_count < (num_of_words+1)/2
        }
    }
}

pub fn day3_main(file_data: &str) {
    // parse to Vec<BinaryWord>
    let (word_len, binary_words) = parse_to_binword_vec(&file_data)
        .unwrap_or_else(|e| {
            if e == 0 {
                eprintln!("File is empty!");
            }
            else {
                eprintln!("Error parsing line {}: invalid format or length", e+1);
            }
            process::exit(1);
        });

    // find most common bit in each column
    let most_common_bit = binary_words
        .iter()
        .fold(vec![0; word_len], |mut acc, word| {
            word.iter().enumerate().for_each(|(j, c)| {
                match c {
                    false => acc[j] -= 1,
                    true => acc[j] += 1
                }
            });
            acc
        })
        .into_iter()
        .map(|count| count > 0)
        .collect::<Vec<bool>>();

    // Part 1
    let gamma = bits_to_u16(&most_common_bit);
    let epsilon = !gamma & ((0x1 << word_len) - 0x1);   // mask irrelevant bits after bitwise not
    println!("Gamma Rate = {}, Epsilon Rate = {}", gamma, epsilon);
    println!("The power consumption of the submarine is {}!", (gamma as u32) * (epsilon as u32));

    // Part 2
    let oxygen_rating = match_bit_criteria(binary_words.clone(), word_len, BitCriteria::Oxygen).unwrap();
    let oxygen_rating = bits_to_u16(&oxygen_rating);
    let co2_rating = match_bit_criteria(binary_words, word_len, BitCriteria::Co2).unwrap();
    let co2_rating = bits_to_u16(&co2_rating);
    println!("\nOxygen Rating: {}, CO2 Rating: {}", oxygen_rating, co2_rating);
    println!("The life support rating of the submarine is {}!", (oxygen_rating as u32) * (co2_rating as u32));
}

fn str_to_binword(s: &str) -> Option<BinaryWord> {
    s.chars().map(|c| {
        match c {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
        }
    })
    .collect::<Option<BinaryWord>>()
}

fn parse_to_binword_vec(data_string: &str) -> Result<(usize, Vec<BinaryWord>), usize> {
    let word_len = data_string
        .lines().nth(0)
        .ok_or_else(|| 0 as u16)?.len();
    let word_vec = data_string
        .lines().enumerate()
        .map(|(i, s)| {
            if s.len() != word_len { return Err(i as usize); }
            str_to_binword(s).ok_or_else(|| i as usize)
        })
        .collect::<Result<Vec<BinaryWord>, usize>>()?;
    Ok((word_len, word_vec))
}

fn bits_to_u16(binary_word: &[bool]) -> u16 {
    let word_len = binary_word.len();
    binary_word.iter().enumerate()
    .fold(0, |acc, (i, is_one)| {
        acc + ((*is_one as u16) << (word_len - 1 - i))
    })
}

fn match_bit_criteria<'a>(mut words: Vec<BinaryWord>, word_len: usize, bit_criteria: BitCriteria)
-> Option<BinaryWord> {
    if word_len == 0 || words.len() == 0 { return None; }
    for j in 0..word_len {
        let last_word = words.last().unwrap().clone();
        let one_count = words
            .iter()
            .fold(0, |acc, word| {
                acc + (*word.iter().nth(j).unwrap() as usize)
            });
        let bit_to_match = bit_criteria.get_bit_to_match(one_count, words.len());

        words.retain(|word| {
            *word.iter().nth(j).unwrap() == bit_to_match
        });

        match words.len() {
            0 => return Some(last_word.clone()),
            1 => return Some(words.pop().unwrap()),
            _ => ()
        }
    }
    Some(words.pop().unwrap())
}