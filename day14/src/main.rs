use regex::Regex;
use std::collections::HashMap;

fn main() {}

fn nanofactory(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)\s(\w+)").unwrap();
    let chemicals = input
        .split('\n')
        .map(|row| {
            let mut chemicals = re
                .captures_iter(row)
                .map(|cap| {
                    let name = &cap[2];
                    let name = name.to_string();
                    let quantity = &cap[1];
                    let quantity = quantity.parse::<i32>().unwrap();

                    (name, quantity)
                })
                .collect::<Vec<(String, i32)>>();

            let mut output = chemicals.pop().unwrap();
            output.1 = -1 * output.1;
            chemicals.push(output.clone());

            (output.0, chemicals.into_iter().collect())
        })
        .collect::<Vec<(String, HashMap<String, i32>)>>();

    let chemicals: HashMap<String, HashMap<String, i32>> = chemicals.into_iter().collect();

    let mut fuel = chemicals.get("FUEL").unwrap().to_owned();
    fuel.remove("FUEL");
    println!("CHEMICALS: {:?}", chemicals);
    println!("INITIAL: {:?}", fuel);
    let mut iter = 0;

    loop {
        for name in fuel.clone().keys() {
            if name == "ORE" {
                continue;
            }
            let mut equation = chemicals[name].clone();
            if equation.contains_key("ORE") {
                continue;
            }

            let min_quantity = equation.remove(name).unwrap().abs();
            let quantity = fuel[name];
            if quantity < min_quantity || quantity % min_quantity != 0 {
                println!(
                    "mismatch quantity for {}, want min {}, got {}",
                    name, min_quantity, quantity
                );
                continue;
            }

            for (chem_name, chem_quantity) in equation {
                let curr_chem = fuel.entry(chem_name).or_insert(0);
                *curr_chem += (chem_quantity / min_quantity) * quantity;
            }
            fuel.remove(name);
        }
        iter += 1;
        if iter > 5000 {
            println!("FORCE BREAK: {:?}", fuel);
            break;
        }

        if fuel.keys().len() == 1 && fuel.contains_key("ORE") {
            break;
        }

        let valid = fuel
            .clone()
            .keys()
            .filter(|&name| name != "ORE")
            .all(|name| chemicals[name].contains_key("ORE"));
        if !valid {
            continue;
        }

        for inp in fuel.clone().keys() {
            match inp.as_ref() {
                "ORE" => {}
                name => {
                    let curr_quantity = fuel[inp] as f32;
                    let min_quantity = chemicals[name][name].abs() as f32;
                    let ore_quantity = chemicals[name]["ORE"] as f32;
                    let quantity_to_add =
                        ((curr_quantity / min_quantity).ceil() * ore_quantity) as i32;
                    let fuel_ore = fuel.entry("ORE".into()).or_insert(0);
                    *fuel_ore += quantity_to_add;
                    fuel.remove(name);
                }
            }
        }
    }

    fuel["ORE"]
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

        let fuel = nanofactory(&input);
        assert_eq!(31, fuel);

        let input = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";

        let fuel = nanofactory(&input);
        assert_eq!(165, fuel);

        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let fuel = nanofactory(&input);
        assert_eq!(13312, fuel);

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

        let fuel = nanofactory(&input);
        assert_eq!(180697, fuel);

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

        let fuel = nanofactory(&input);
        assert_eq!(2210736, fuel);
    }
}
