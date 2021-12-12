fn contains_digit(s: &str, other: &str) -> bool {
    other.chars().all(|c| s.contains(c))
}
fn is_same_digit(s: &str, other: &str) -> bool {
    if s.len() != other.len() { return false; }
    other.chars().all(|c| s.contains(c))
}

fn decode_entry_string(s: &str) -> Option<u16> {
    let mut sections = s.split('|');
    let mut digit_patterns: [Option<&str>; 10] = [None; 10];

    // put the known words into digit_patterns and collect the remaning unknown words
    let todo = sections.next()?
        .split_whitespace().filter_map(|s| {
            let mut ret_val = None;
            match s.len() {
                2 => digit_patterns[1] = Some(s),
                3 => digit_patterns[7] = Some(s),
                4 => digit_patterns[4] = Some(s),
                7 => digit_patterns[8] = Some(s),
                _ => ret_val = Some(s)
            }
            ret_val
        })
        .collect::<Vec<_>>();
    let output_strings = sections.next()?.split_whitespace().collect::<Vec<_>>();
    if output_strings.len() != 4 { return None; }

    // check if '1', '4', '7', '8' were successfully found
    for i in [1, 4, 7, 8] {
        digit_patterns[i]?;
    }

    // partition by checking if the digit is contains '7'
    let (mut contains_7, mut todo): (Vec<_>, Vec<_>) = todo.into_iter().partition(|&s| {
        contains_digit(s, digit_patterns[7].unwrap())
    });
    if contains_7.len() != 3 { return None; }   // precisely 3 digits satisfies the condition
    if todo.len() != 3 { return None; }   // precisely 3 digits satisfies the condition


    // contains 7 and length 5 ---> '3'
    let pos = contains_7.iter().position(|&s| s.len() == 5)?;
    digit_patterns[3] = Some(contains_7[pos]);
    contains_7.remove(pos);

    // contains 7 and 4 ---> '9'
    let pos = contains_7.iter().position(|&s| contains_digit(s, digit_patterns[4].unwrap()))?;
    digit_patterns[9] = Some(contains_7[pos]);
    contains_7.remove(pos);

    // last one containing 7 ---> '0'
    digit_patterns[0] = Some(contains_7.pop().unwrap());    // assumes the words actually represent the 10 digits

    // doesn't contain '7' and length 6 ---> '6'
    let pos = todo.iter().position(|&s| s.len() == 6)?;
    digit_patterns[6] = Some(todo[pos]);
    todo.remove(pos);

    // doesn't contain '7' and contained in '9' ---> '5'
    let pos = todo.iter().position(|&s| contains_digit(digit_patterns[9].unwrap(), s))?;
    digit_patterns[5] = Some(todo[pos]);
    todo.remove(pos);

    // last one ---> '2'
    digit_patterns[2] = Some(todo.pop().unwrap());  // assumes the words actually represent the 10 digits

    // get output digits
    let mut output = 0;
    // println!("digit_patterns = {:?}", digit_patterns);
    // println!("output_strings: {:?}", output_strings);
    for i in 0..output_strings.len() {
        let num = digit_patterns.iter().position(|word| {
            is_same_digit(word.unwrap(), output_strings[output_strings.len()-1-i])
        })?;
        output += (num as u16) * u16::pow(10, i as u32);
    }

    Some(output)
}


pub fn day8_main(file_data: &str) -> (u16, u32) {
    // Part 1
    let output_digits = file_data.split(&['|', '\n'][..])
        .skip(1).step_by(2).map(|s| s.split_whitespace());

    let part1_count = output_digits.flatten()
        .fold(0, |acc, s| {
            acc + match s.len() {
                 2 | 3 | 4 | 7 => 1,
                _ => 0
            }
        });
    println!("There are {} occurences of '1', '4', '7' or '8' in the output digits.", part1_count);

    // Part 2
    let part2_count = file_data.lines()
        .enumerate()
        .fold(0, |acc, (i, line)| {
            acc + decode_entry_string(line)
                .unwrap_or_else(|| panic!("Unable to decode entry on line {}!", i+1)) as u32
        });
    println!("The sum of all output digits is {}.", part2_count);

    (part1_count, part2_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(day8_main(test_data), (26, 61229));
    }
}