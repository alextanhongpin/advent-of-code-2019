use regex::Regex;
use std::collections::HashMap;

fn main() {}

fn parse(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let re = Regex::new(r"(\d+)\s(\w+)").unwrap();
    let reactions = input
        .split('\n')
        .map(|row| {
            let mut reactions = re
                .captures_iter(row)
                .map(|cap| {
                    let (name, quantity) = (&cap[2], &cap[1]);
                    (name.to_string(), quantity.parse::<i32>().unwrap())
                })
                .collect::<Vec<(String, i32)>>();

            let mut output = reactions.pop().unwrap();
            output.1 = -1 * output.1;
            reactions.push(output.clone());

            (output.0, reactions.into_iter().collect())
        })
        .collect::<Vec<(String, HashMap<String, i32>)>>();

    reactions.into_iter().collect()
}

fn nanofactory(input: &str) -> i32 {
    let reactions = parse(input);
    let mut have: HashMap<String, i32> = HashMap::new();
    let mut want: HashMap<String, i32> = HashMap::new();
    want.insert("FUEL".into(), 1);

    let mut iter = 0;

    loop {
        // 1 FUEL = 7 A + 1 E
        for key in want.clone().keys() {
            if key == "ORE" {
                continue;
            }
            let want_qty = want[key];
            let chem_qty = reactions[key][key].abs();

            if want_qty >= chem_qty {
                want.remove(key);
                let mut chem_reaction = reactions[key].clone();
                chem_reaction.remove(key);

                for (chem_name, chem_qty) in chem_reaction {
                    if chem_name == "ORE" {
                        continue;
                    }
                    let chem = want.entry(chem_name).or_insert(0);
                    *chem += chem_qty * want_qty;
                }
            } else {
                println!("key: {}, want: {}, chem: {}", key, want_qty, chem_qty);
                //println!("key: {}, have: {:?}, want: {:?}", key, have, want);
                //println!("eq: {:?}", reactions[key]);
            }
        }

        for key in want.clone().keys() {
            if key == "ORE" {
                continue;
            }
            if !reactions[key].contains_key("ORE") {
                continue;
            }
            let have_qty = have.get(key).unwrap_or(&0);
            let want_qty = want[key];

            if *have_qty < want_qty {
                let prod_qty = reactions[key][key].abs();
                let ratio = (want_qty as f32 / prod_qty as f32).ceil() as i32;
                let ore_qty = reactions[key]["ORE"];

                let have_entry = have.entry(key.clone()).or_insert(0);
                *have_entry += prod_qty * ratio;

                let want_entry = want.entry("ORE".clone().to_string()).or_insert(0);
                *want_entry += ore_qty * ratio;
            }

            let want_entry = want.entry(key.to_string()).or_insert(0);
            *want_entry -= want_qty;
            if *want_entry == 0 {
                want.remove(key);
            }

            let have_entry = have.entry(key.clone()).or_insert(0);
            *have_entry -= want_qty;
        }

        if want.keys().len() == 1 && want.contains_key("ORE") {
            break;
        }
        iter += 1;
        if iter > 10 {
            break;
        }
    }

    want["ORE"]
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

        let want = nanofactory(&input);
        assert_eq!(31, want);

        let input = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";

        let want = nanofactory(&input);
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

        let want = nanofactory(&input);
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

        let want = nanofactory(&input);
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

        let want = nanofactory(&input);
        assert_eq!(2210736, want);
    }
}
