use fnv::FnvHashMap;

#[derive(Debug, Default)]
pub struct FishSchool {
    // This is a map of the days a fish has to reproduce and the number of fish in that stage
    school: FnvHashMap<usize, usize>,
}

impl FishSchool {
    fn new(input: &[usize]) -> Self {
        let mut school = FishSchool {
            school: FnvHashMap::default(),
        };

        // We know we only can have 9 states (0 to 8) so make them all now
        for x in 0..=8 {
            school.school.insert(x, 0);
        }

        for ea in input {
            school.school.entry(*ea).and_modify(|val| *val += 1);
        }

        school
    }

    fn simulate_day(&mut self) {
        let reproducing_fish = self
            .school
            .get(&0)
            .expect("Error while getting reproducing fish count")
            .to_owned();

        let mut prev_age_count = self
            .school
            .get(&8)
            .expect("Error getting previous age count for fish")
            .to_owned();
        for x in (0..=7).rev() {
            prev_age_count = self
                .school
                .insert(x, prev_age_count)
                .expect("Inserting to fish_school failed");
        }

        self.school
            .entry(8)
            .and_modify(|val| *val = reproducing_fish);
        self.school
            .entry(6)
            .and_modify(|val| *val += reproducing_fish);
    }

    fn count_fish(&self) -> usize {
        self.school.values().sum()
    }
}

pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .into_iter()
        .map(|x| x.parse().expect("Error parsing fish days to reproduce"))
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    let mut school_of_fish = FishSchool::new(input);

    for _ in 0..80 {
        school_of_fish.simulate_day();
    }

    school_of_fish.count_fish()
}

pub fn part2(input: &[usize]) -> usize {
    let mut school_of_fish = FishSchool::new(input);

    for _ in 0..256 {
        school_of_fish.simulate_day();
    }

    school_of_fish.count_fish()
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

    test!(part1, 5934);
    test!(part2, 26984457539);
}
