use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();
    assert_eq!(301997, nanofactory(&input, 1));
    assert_eq!(6216589, binary_search(&input, 1e12 as i64));

    Ok(())
}

fn parse(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let re = Regex::new(r"(\d+)\s(\w+)").unwrap();
    let reactions = input
        .split('\n')
        .map(|row| {
            let mut reactions = re
                .captures_iter(row)
                .map(|cap| {
                    let (name, quantity) = (&cap[2], &cap[1]);
                    (name.to_string(), quantity.parse::<i64>().unwrap())
                })
                .collect::<Vec<(String, i64)>>();

            let mut output = reactions.pop().unwrap();
            output.1 = -1 * output.1;
            reactions.push(output.clone());

            (output.0, reactions.into_iter().collect())
        })
        .collect::<Vec<(String, HashMap<String, i64>)>>();

    reactions.into_iter().collect()
}

fn nanofactory(input: &str, fuel: i64) -> i64 {
    let reactions = parse(input);
    let mut have: HashMap<String, i64> = HashMap::new();
    let mut want: HashMap<String, i64> = HashMap::new();
    want.insert("FUEL".into(), fuel);

    let mut ore = 0;
    while want.keys().len() > 0 {
        let item = want.keys().cloned().next().unwrap();
        if want.get(&item).unwrap_or(&0) <= have.get(&item).unwrap_or(&0) {
            let chem_have = have.entry(item.to_string()).or_insert(0);
            *chem_have -= want.get(&item).unwrap_or(&0);
            want.remove(&item);
            continue;
        }

        let num_want = want.get(&item).unwrap_or(&0) - have.get(&item).unwrap_or(&0);
        have.remove(&item);
        want.remove(&item);
        let num_produced = reactions[&item][&item].abs();

        let num_reactions = if num_want % num_produced == 0 {
            num_want / num_produced
        } else {
            num_want / num_produced + 1
        };

        let chem_have = have.entry(item.clone()).or_insert(0);
        *chem_have += (num_reactions * num_produced) - num_want;
        for (chem, quantity) in reactions[&item].clone() {
            if quantity < 0 {
                continue;
            }
            if chem == "ORE" {
                ore += quantity * num_reactions;
            } else {
                let chem_want = want.entry(chem).or_insert(0);
                *chem_want += quantity * num_reactions;
            }
        }
    }

    ore
}

fn binary_search(input: &str, ore_required: i64) -> i64 {
    let mut min: i64 = 1;
    let mut max: i64 = ore_required;

    while min < max {
        let mid = min + (max - min) / 2;
        let ore_produced = nanofactory(input, mid);
        if ore_produced > ore_required {
            max = mid - 1
        } else if ore_produced < ore_required {
            min = mid + 1
        } else {
            break;
        }
    }

    min + (max - min) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

        let want = nanofactory(&input, 1);
        assert_eq!(31, want);

        let input = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";

        let want = nanofactory(&input, 1);
        assert_eq!(165, want);

        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let want = nanofactory(&input, 1);
        assert_eq!(13312, want);

        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        let want = nanofactory(&input, 1);
        assert_eq!(180697, want);

        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

        let want = nanofactory(&input, 1);
        assert_eq!(2210736, want);
    }

    #[test]
    fn part2() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let want = binary_search(&input, 1e12 as i64);
        assert_eq!(82892753, want);

        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        let want = binary_search(&input, 1e12 as i64);
        assert_eq!(5586022, want);

        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

        let want = binary_search(&input, 1e12 as i64);
        assert_eq!(460664, want);
    }
}
