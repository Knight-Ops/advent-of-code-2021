use fnv::FnvHashMap;

#[derive(Debug, Default, Clone)]
pub struct Polymerization<'a> {
    polymer_template: String,
    rules: FnvHashMap<&'a [u8], &'a [u8]>,
}

pub fn input_generator(input: &str) -> Polymerization {
    let mut split_input = input.split("\n\n");

    let polymer_template = split_input
        .next()
        .expect("Starting polymer template does not exist in input");

    let mut rules = FnvHashMap::default();
    split_input
        .next()
        .expect("Pair insertion rules do not exist in input")
        .lines()
        .into_iter()
        .for_each(|line| {
            let mut rule = line.split(" -> ");

            let key = rule.next().expect("No key in rule entry");
            let value = rule.next().expect("No value in rule entry");

            rules.insert(key.as_bytes(), value.as_bytes());
        });

    Polymerization {
        polymer_template: polymer_template.to_string(),
        rules,
    }
}

pub fn part1(input: &Polymerization) -> usize {
    let mut poly_chain_map_old: FnvHashMap<Vec<u8>, usize> = FnvHashMap::default();
    let mut poly_chain_map_new: FnvHashMap<Vec<u8>, usize> = FnvHashMap::default();

    input
        .polymer_template
        .as_bytes()
        .windows(2)
        .for_each(|byte_slice| {
            *poly_chain_map_old.entry(byte_slice.to_owned()).or_insert(0) += 1;
        });

    for _ in 0..10 {
        poly_chain_map_old.iter().for_each(|(byte_slice, &count)| {
            let insertion = input
                .rules
                .get(byte_slice.as_slice())
                .expect("Rule does not exist in Polymerization structure");

            *poly_chain_map_new
                .entry([byte_slice[0], insertion[0]].to_vec())
                .or_insert(0) += count;
            *poly_chain_map_new
                .entry([insertion[0], byte_slice[1]].to_vec())
                .or_insert(0) += count;
        });

        poly_chain_map_old = poly_chain_map_new.clone();
        poly_chain_map_new.clear();
    }

    let mut char_map = FnvHashMap::default();
    let first_byte = input
        .polymer_template
        .bytes()
        .next()
        .expect("Couldn't get first byte of polymer_template")
        .to_owned();
    char_map.insert(&first_byte, 1);
    poly_chain_map_old.iter().for_each(|(key, val)| {
        key.iter().skip(1).for_each(|letter| {
            *char_map.entry(letter).or_insert(0) += val;
        })
    });

    let mut max = None;
    let mut min = None;

    char_map.values().for_each(|val| {
        if let Some(curr_max) = max {
            if val > curr_max {
                max = Some(val);
            }
        } else {
            max = Some(val);
        }

        if let Some(curr_min) = min {
            if val < curr_min {
                min = Some(val);
            }
        } else {
            min = Some(val);
        }
    });

    max.expect("Max was never populated with Some!")
        - min.expect("Min was never populated with Some!")
}

pub fn part1_slow(input: &Polymerization) -> usize {
    let mut poly_chain = input.polymer_template.as_bytes().to_vec();

    for _ in 0..10 {
        let last_char = poly_chain[poly_chain.len() - 1];
        poly_chain = poly_chain
            .windows(2)
            .map(|byte_slice| {
                let insertion = input
                    .rules
                    .get(byte_slice)
                    .expect("Rule does not exist in Polymerization structure");

                [byte_slice[0], insertion[0]]
            })
            .flatten()
            .collect();

        poly_chain.push(last_char);
    }

    let mut char_map = FnvHashMap::default();
    poly_chain.iter().for_each(|x| {
        *char_map.entry(x).or_insert(0) += 1;
    });

    let mut max = None;
    let mut min = None;

    char_map.values().for_each(|val| {
        if let Some(curr_max) = max {
            if val > curr_max {
                max = Some(val);
            }
        } else {
            max = Some(val);
        }

        if let Some(curr_min) = min {
            if val < curr_min {
                min = Some(val);
            }
        } else {
            min = Some(val);
        }
    });

    max.expect("Max was never populated with Some!")
        - min.expect("Min was never populated with Some!")
}

pub fn part2(input: &Polymerization) -> usize {
    let mut poly_chain_map_old: FnvHashMap<Vec<u8>, usize> = FnvHashMap::default();
    let mut poly_chain_map_new: FnvHashMap<Vec<u8>, usize> = FnvHashMap::default();

    input
        .polymer_template
        .as_bytes()
        .windows(2)
        .for_each(|byte_slice| {
            *poly_chain_map_old.entry(byte_slice.to_owned()).or_insert(0) += 1;
        });

    for _ in 0..40 {
        poly_chain_map_old.iter().for_each(|(byte_slice, &count)| {
            let insertion = input
                .rules
                .get(byte_slice.as_slice())
                .expect("Rule does not exist in Polymerization structure");

            *poly_chain_map_new
                .entry([byte_slice[0], insertion[0]].to_vec())
                .or_insert(0) += count;
            *poly_chain_map_new
                .entry([insertion[0], byte_slice[1]].to_vec())
                .or_insert(0) += count;
        });

        poly_chain_map_old = poly_chain_map_new.clone();
        poly_chain_map_new.clear();
    }

    let mut char_map = FnvHashMap::default();
    let first_byte = input
        .polymer_template
        .bytes()
        .next()
        .expect("Couldn't get first byte of polymer_template")
        .to_owned();
    char_map.insert(&first_byte, 1);
    poly_chain_map_old.iter().for_each(|(key, val)| {
        key.iter().skip(1).for_each(|letter| {
            *char_map.entry(letter).or_insert(0) += val;
        })
    });

    let mut max = None;
    let mut min = None;

    char_map.values().for_each(|val| {
        if let Some(curr_max) = max {
            if val > curr_max {
                max = Some(val);
            }
        } else {
            max = Some(val);
        }

        if let Some(curr_min) = min {
            if val < curr_min {
                min = Some(val);
            }
        } else {
            min = Some(val);
        }
    });

    max.expect("Max was never populated with Some!")
        - min.expect("Min was never populated with Some!")
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

    test!(part1, 1588);
    test!(part1_slow, 1588);
    test!(part2, 2188189693529);
}
