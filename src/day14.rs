use std::str;
use std::fmt;
use std::iter;
use std::hash::Hash;
use std::error::Error;
use std::collections::HashMap;

type Element = char;
type Pair = [char; 2];
type RuleMap = HashMap<[char; 2], Element>;

#[derive(Debug)]
enum ParseRuleError {
    FormatError, InvalidPattern, InvalidElement
}
impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseRuleError::FormatError => write!(f, "invalid insertion rule format"),
            ParseRuleError::InvalidPattern => write!(f, "insertion location must be a pair of elements"),
            ParseRuleError::InvalidElement => write!(f, "insert element must be a char"),
        }
    }
}
impl Error for ParseRuleError {}

#[derive(PartialEq, Eq, Hash)]
struct Rule(Pair, Element);
impl str::FromStr for Rule {
    type Err = ParseRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split("->");
        if let (Some(pattern), Some(element), None) = (tokens.next(), tokens.next(), tokens.next()) {
            let pattern = pattern.trim();
            if pattern.len() != 2 { return Err(ParseRuleError::InvalidPattern); }
            let pattern = [pattern.chars().next().unwrap(), pattern.chars().nth(1).unwrap()];

            let element = element.trim();
            if element.len() != 1 { return Err(ParseRuleError::InvalidElement); }
            let element = element.chars().next().unwrap();

            Ok(Rule(pattern, element))
        }
        else { Err(ParseRuleError::FormatError) }
    }
}

struct CountMap<T: Eq + Hash>(HashMap<T, u64>);
impl<T: Eq + Hash> CountMap<T> {
    fn new() -> Self {
        CountMap(HashMap::new())
    }
    fn increment_count(&mut self, key: T, count: u64) {
        *self.0.entry(key).or_insert(0) += count;
    }
}
impl<T: Eq + Hash> iter::Extend<(T, u64)> for CountMap<T> {
    fn extend<I: IntoIterator<Item=(T, u64)>>(&mut self, iter: I) {
        for (elem, count) in iter {
            self.increment_count(elem, count);
        }
    }
}

struct Template {
    pair_count: CountMap<Pair>, element_count: CountMap<Element>
}
impl str::FromStr for Template {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair_count = CountMap::new();
        let mut element_count = CountMap::new();

        if let Some(c) = s.chars().next() { element_count.increment_count(c, 1); }
        for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
            pair_count.increment_count([c1, c2], 1);
            element_count.increment_count(c2, 1);
        }
        Ok(Template { pair_count, element_count })
    }
}

#[derive(Debug)]
enum ParsePolymerFinderError {
    FormatError, InvalidRule((usize, ParseRuleError)), DuplicatePattern
}
impl fmt::Display for ParsePolymerFinderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePolymerFinderError::FormatError => write!(f, "invalid instructions format"),
            ParsePolymerFinderError::DuplicatePattern => write!(f, "duplicate pattern found"),
            ParsePolymerFinderError::InvalidRule((i, _)) => write!(f, "insertion rule {} is invalid", i),
        }
    }
}
impl From<(usize, ParseRuleError)> for ParsePolymerFinderError {
    fn from(e: (usize, ParseRuleError)) -> Self {
        ParsePolymerFinderError::InvalidRule(e)
    }
}
impl Error for ParsePolymerFinderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsePolymerFinderError::InvalidRule((_, e)) => Some(e),
            _ => None,
        }
    }
}

struct PolymerFinder {
    template: Template, rules: RuleMap
}
impl PolymerFinder {
    fn apply_insertion(&mut self) {
        let mut temp = CountMap::new();
        self.template.pair_count.0.retain(|pair, count| {
            match self.rules.get(pair) {
                None => true,
                Some(&c) => {
                    temp.increment_count([pair[0], c], *count);
                    temp.increment_count([c, pair[1]], *count);
                    self.template.element_count.increment_count(c, *count);
                    false
                }
            }
        });
        self.template.pair_count.extend(temp.0);
    }
}
impl str::FromStr for PolymerFinder {
    type Err = ParsePolymerFinderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let section_delimiter =
            match s.chars().nth(s.lines().next().ok_or(ParsePolymerFinderError::FormatError)?.len()) {
                None => return Ok(PolymerFinder {
                    template: s.parse().unwrap(), rules: RuleMap::new()
                }),
                Some('\r') => "\r\n\r\n",
                _ => "\n\n",
            };

        let mut sections = s.split(section_delimiter);
        if let (Some(template), Some(rules), None) = (sections.next(),sections.next(),sections.next()) {
            let rule_map = rules.lines().enumerate()
                .map(|(i, s)| {
                    let rule = s.parse::<Rule>().map_err(|e| (i+1, e))?;
                    Ok((rule.0, rule.1))
                })
                .collect::<Result<RuleMap, ParsePolymerFinderError>>()?;
            if rules.lines().count() != rule_map.len() { return Err(ParsePolymerFinderError::DuplicatePattern); }
            let template = template.parse().unwrap();

            Ok(PolymerFinder { template, rules: rule_map })
        }
        else { Err(ParsePolymerFinderError::FormatError) }
    }
}


pub fn day14_main(file_data: &str) -> (u64, u64) {
    let mut polymer_finder = file_data.parse::<PolymerFinder>()
        .unwrap_or_else(|e| {
            if let Some(src) = e.source() {
                panic!("Error parsing instructions! : {}, caused by \"{}\"", e, src);
            }
            else { panic!("Error parsing instructions! : {}", e); }
        });

    // Part 1
    for _ in 1..=10 { polymer_finder.apply_insertion(); }
    let max_count = polymer_finder.template.element_count.0.values().max().unwrap();
    let min_count = polymer_finder.template.element_count.0.values().min().unwrap();
    let part1_answer = max_count - min_count;
    println!("[Part 1] After 10 insertions, The differnce is {}.", part1_answer);

    // Part 2
    for _ in 1..=30 { polymer_finder.apply_insertion(); }
    let max_count = polymer_finder.template.element_count.0.values().max().unwrap();
    let min_count = polymer_finder.template.element_count.0.values().min().unwrap();
    let part2_answer = max_count - min_count;
    println!("[Part 2] After 40 insertions, The differnce is {}.", part2_answer);

    (part1_answer, part2_answer)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C";

        assert_eq!(day14_main(test_data), (1588, 2188189693529));
    }
}