use fnv::{FnvHashMap, FnvHashSet};
use lazy_static::lazy_static;

lazy_static! {
    static ref STR_TO_VAL_ORACLE: FnvHashMap<&'static str, char> = {
        let mut map = FnvHashMap::default();
        map.insert("abcefg", '0');
        map.insert("cf", '1');
        map.insert("acdeg", '2');
        map.insert("acdfg", '3');
        map.insert("bcdf", '4');
        map.insert("abdfg", '5');
        map.insert("abdefg", '6');
        map.insert("acf", '7');
        map.insert("abcdefg", '8');
        map.insert("abcdfg", '9');
        map
    };
}

#[derive(Debug, Default, Clone)]
pub struct NoteEntry<'a> {
    signal_patterns: Vec<&'a str>,
    output_values: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub enum SegmentSolution {
    Solved(char),
    Possible(Vec<char>),
}

impl SegmentSolution {
    fn get_solved(&self) -> char {
        match self {
            Self::Solved(c) => *c,
            _ => panic!("Called get_solved on unsolved entry"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SegmentDisplay {
    a: Option<SegmentSolution>,
    b: Option<SegmentSolution>,
    c: Option<SegmentSolution>,
    d: Option<SegmentSolution>,
    e: Option<SegmentSolution>,
    f: Option<SegmentSolution>,
    g: Option<SegmentSolution>,
}

impl SegmentDisplay {
    fn set_possible(&mut self, jumbled_value: char, possible_corrected_values: &[char]) {
        match jumbled_value {
            'a' => {
                self.a = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'b' => {
                self.b = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'c' => {
                self.c = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'd' => {
                self.d = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'e' => {
                self.e = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'f' => {
                self.f = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            'g' => {
                self.g = Some(SegmentSolution::Possible(
                    possible_corrected_values.to_vec(),
                ))
            }
            _ => unreachable!(
                "Jumbled value provided that does not exist in the seven-segment display"
            ),
        }
    }

    fn set_solved(&mut self, jumbled_value: char, solved_value: char) {
        let solution = Some(SegmentSolution::Solved(solved_value));

        match jumbled_value {
            'a' => self.a = solution,
            'b' => self.b = solution,
            'c' => self.c = solution,
            'd' => self.d = solution,
            'e' => self.e = solution,
            'f' => self.f = solution,
            'g' => self.g = solution,
            _ => unreachable!(
                "Jumbled value provided that does not exist in the seven-segment display"
            ),
        }
    }

    fn is_already_populated(&self, c: char) -> bool {
        match c {
            'a' => self.a.is_some(),
            'b' => self.b.is_some(),
            'c' => self.c.is_some(),
            'd' => self.d.is_some(),
            'e' => self.e.is_some(),
            'f' => self.f.is_some(),
            'g' => self.g.is_some(),
            _ => unreachable!("Value provided that does not exist in the seven-segment display"),
        }
    }

    fn get(&self, c: char) -> &Option<SegmentSolution> {
        match c {
            'a' => &self.a,
            'b' => &self.b,
            'c' => &self.c,
            'd' => &self.d,
            'e' => &self.e,
            'f' => &self.f,
            'g' => &self.g,
            _ => unreachable!("Char provided that doesn't exist in SegmentDisplay"),
        }
    }

    fn reduce(&mut self, c: char) {
        for each in [
            &mut self.a,
            &mut self.b,
            &mut self.c,
            &mut self.d,
            &mut self.e,
            &mut self.f,
            &mut self.g,
        ] {
            if let Some(SegmentSolution::Possible(vec)) = each {
                if vec.contains(&c) && vec.len() == 2 {
                    let reduction_solve = {
                        if vec[0] == c {
                            vec[1]
                        } else {
                            vec[0]
                        }
                    };

                    *each = Some(SegmentSolution::Solved(reduction_solve));
                }
            }
        }
    }

    fn solve(&mut self, note: &NoteEntry) {
        self.solve_a(note);

        self.populate_b_d(note);

        self.solve_g_e(note);

        self.solve_b_d_c_f(note);
    }

    fn solve_a(&mut self, note: &NoteEntry) {
        let mut signals: Vec<&&str> = note
            .signal_patterns
            .iter()
            .filter(|signal| signal.len() == 2 || signal.len() == 3)
            .collect();

        signals.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        for sig in signals {
            match sig.len() {
                2 => {
                    // We have a "1" being displayed here so we want to set possible c and f
                    for c in sig.chars() {
                        self.set_possible(c, &['c', 'f'])
                    }
                }
                3 => {
                    // We have a "7" being displayed here, so we can solve for a
                    for c in sig.chars() {
                        if !self.is_already_populated(c) {
                            self.set_solved(c, 'a')
                        }
                    }
                }
                _ => unreachable!("Got a number we weren't expecting in solve_a"),
            }
        }
    }

    fn populate_b_d(&mut self, note: &NoteEntry) {
        let signals: Vec<&&str> = note
            .signal_patterns
            .iter()
            .filter(|signal| signal.len() == 4)
            .collect();

        for sig in signals {
            // We have a "4" being displayed here so we want to set possible b and d
            for c in sig.chars() {
                if !self.is_already_populated(c) {
                    self.set_possible(c, &['b', 'd'])
                }
            }
        }
    }

    fn solve_g_e(&mut self, note: &NoteEntry) {
        let mut signals = note
            .signal_patterns
            .iter()
            .filter(|signal| signal.len() == 5);

        // We don't know if this is a "2" or "3|5", ideally we want a "3|5" so we can solve for g first
        let first_entry = signals
            .next()
            .expect("Could not get first signal in solve_g_e");
        if first_entry
            .chars()
            .filter(|x| !self.is_already_populated(*x))
            .count()
            == 1
        {
            for c in first_entry.chars() {
                if !self.is_already_populated(c) {
                    self.set_solved(c, 'g')
                }
            }

            // We can just cheat here because regardless if we originally got a "3" or "5", we can
            // just go through all of them since, if we happen to got companion number, all the areas
            // will be populated already
            signals.for_each(|x| {
                for c in x.chars() {
                    if !self.is_already_populated(c) {
                        self.set_solved(c, 'e')
                    }
                }
            })
        } else {
            // Getting this we know we have a "3" or "5" so we can solve this first
            let second_entry = signals
                .next()
                .expect("Could not get second signal in solve_g_e");
            for c in second_entry.chars() {
                if !self.is_already_populated(c) {
                    self.set_solved(c, 'g')
                }
            }

            // Now we can solve e, since we already have the "2"
            for c in first_entry.chars() {
                if !self.is_already_populated(c) {
                    self.set_solved(c, 'e')
                }
            }
        }
    }

    fn solve_b_d_c_f(&mut self, note: &NoteEntry) {
        let signals: Vec<&&str> = note
            .signal_patterns
            .iter()
            .filter(|signal| signal.len() == 6)
            .collect();

        let all_chars = "abcdefg";
        for sig in signals {
            let mut missing_seg = '\0';
            for c in all_chars.chars() {
                if !sig.contains(c) {
                    missing_seg = c;
                    break;
                }
            }

            match self.get(missing_seg) {
                // This means we got "9", which we don't really care about since we already solved the missing segment
                Some(SegmentSolution::Solved('e')) => {}
                Some(SegmentSolution::Possible(vec)) => {
                    if vec.contains(&'c') {
                        // This means we got "6", which means we can solve c, and by means of elimination also f
                        self.set_solved(missing_seg, 'c');
                        self.reduce('c');
                    } else if vec.contains(&'d') {
                        // This means we got "0", which means we can solve for d, and by elimination also b
                        self.set_solved(missing_seg, 'd');
                        self.reduce('d');
                    } else {
                        unreachable!("Only c and d should be possible missing values here")
                    }
                }
                _ => unreachable!("We should have data for all of these segments at this point"),
            }
        }
    }

    fn get_digit(&self, output_str: &str) -> char {
        let mut decoded = Vec::new();
        for c in output_str.chars() {
            match c {
                'a' => decoded.push(self.a.as_ref().expect("self.a is None").get_solved()),
                'b' => decoded.push(self.b.as_ref().expect("self.b is None").get_solved()),
                'c' => decoded.push(self.c.as_ref().expect("self.c is None").get_solved()),
                'd' => decoded.push(self.d.as_ref().expect("self.d is None").get_solved()),
                'e' => decoded.push(self.e.as_ref().expect("self.e is None").get_solved()),
                'f' => decoded.push(self.f.as_ref().expect("self.f is None").get_solved()),
                'g' => decoded.push(self.g.as_ref().expect("self.g is None").get_solved()),
                _ => unreachable!("This character should not exist in the output_str"),
            }
        }

        decoded.sort();
        let decoded_str = decoded.into_iter().collect::<String>();
        *STR_TO_VAL_ORACLE
            .get(decoded_str.as_str())
            .expect("Decoded &str did not exist in STR_TO_VAL_ORACLE")
    }
}

pub fn input_generator(input: &str) -> Vec<NoteEntry> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut note = line.split(" | ");

            let signal_patterns: Vec<&str> = note
                .next()
                .expect("First half of note entry doesn't exist")
                .split_whitespace()
                .collect();
            let output_values: Vec<&str> = note
                .next()
                .expect("Second half of note entry doesn't exist")
                .split_whitespace()
                .collect();

            NoteEntry {
                signal_patterns,
                output_values,
            }
        })
        .collect()
}

pub fn part1(input: &[NoteEntry]) -> usize {
    input
        .iter()
        .map(|note_entry| {
            note_entry
                .output_values
                .iter()
                .filter(|&&x| {
                    // These are the "unique values" that we are looking for corresponding to
                    // "1", "7", "4", and "8"
                    if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {
                        true
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

pub fn part2(input: &[NoteEntry]) -> usize {
    input
        .iter()
        .map(|note_entry| {
            let mut segment_display = SegmentDisplay::default();

            segment_display.solve(note_entry);

            let mut digit_display = Vec::new();
            for output in &note_entry.output_values {
                digit_display.push(segment_display.get_digit(output));
            }
            digit_display
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .expect("Error while parsing digit display")
        })
        .sum::<usize>()
}

// This is an infinitely simpler solution, and it happens to also be faster. It relies on being able to find
// the "unique" values, and using intersections of hashsets to quickly weed out values being displayed
pub fn part2_alternate(input: &[NoteEntry]) -> usize {
    input
        .iter()
        .map(|note_entry| {
            let mut one_hs = FnvHashSet::default();
            let mut four_hs = FnvHashSet::default();
            let mut seven_hs = FnvHashSet::default();
            let mut eight_hs = FnvHashSet::default();

            for signal in &note_entry.signal_patterns {
                if signal.len() == 2 {
                    one_hs = signal.chars().collect();
                } else if signal.len() == 3 {
                    seven_hs = signal.chars().collect();
                } else if signal.len() == 4 {
                    four_hs = signal.chars().collect();
                } else if signal.len() == 7 {
                    eight_hs = signal.chars().collect();
                }
            }

            note_entry.output_values.iter().map(|x| {
                if x.len() == 2 {
                    '1'
                } else if x.len() == 3 {
                    '7'
                } else if x.len() == 4 {
                    '4'
                } else if x.len() == 7 {
                    '8'
                } else {
                    let curr_hs : FnvHashSet<char> = x.chars().collect();

                    if x.len() == 5 {
                        if curr_hs.intersection(&one_hs).count() == 2 {
                            '3'
                        } else {
                            let handicap_hs: FnvHashSet<char> = seven_hs.difference(&four_hs).into_iter().cloned().collect();

                            if curr_hs.intersection(&handicap_hs).count() == 3 {
                                '5'
                            } else {
                                '2'
                            }
                        }
                    } else if x.len() == 6 {
                        if curr_hs.intersection(&seven_hs).count() == 2 {
                            '6'
                        } else if curr_hs.intersection(&four_hs).count() == 4 {
                            '9'
                        } else {
                            '0'
                        }
                    } else {
                        unreachable!("No numbers should be left unparsed");
                    }
                }
            }).collect::<String>().parse::<usize>().expect("Cannot parse output value")
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 26);
    test!(part2, 61229);
    test!(part2_alternate, 61229);
}
