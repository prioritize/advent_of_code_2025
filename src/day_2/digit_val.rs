use anyhow::Result;
use anyhow::anyhow;
use std::{fs, io::Read};

pub struct DigitRange {
    start: u64,
    end: u64,
}
impl DigitRange {
    pub fn find_invalid_part_1(&self) -> Option<Vec<u64>> {
        let mut invalids = Vec::new();
        for r in self.start..=self.end {
            let str_rep = r.to_string();
            if str_rep.len() % 2 != 0 {
                continue;
            }
            let half = str_rep.len() / 2;
            if str_rep[0..half] == str_rep[half..] {
                invalids.push(r);
            }
        }
        if invalids.is_empty() {
            None
        } else {
            println!("{invalids:?}");
            Some(invalids)
        }
    }
}
pub fn parse_input(f_name: &str) -> Result<Vec<DigitRange>> {
    let mut file = fs::File::open(f_name)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf)?;
    let ranges = buf
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let ranges = ranges
        .iter()
        .map(|x| x.split("-").map(|v| v.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let mut out = vec![];
    for r in &ranges {
        let start = match r[0].trim().parse::<u64>() {
            Ok(v) => v,
            Err(_) => return Err(anyhow!("Couldn't parse r[0]")),
        };
        println!("{}", r[1]);
        let end = match r[1].trim().parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to parse {} into a u64", r[1]);
                return Err(anyhow!("Couldn't parse r[1]"));
            }
        };
        out.push(DigitRange {
            start: r[0].trim().parse::<u64>()?,
            end: r[1].trim().parse::<u64>()?,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_test_parse_input() -> Result<()> {
        let ranges = parse_input("input/day_2_input.txt")?;
        let invalids = ranges
            .iter()
            .map(|r| r.find_invalid_part_1())
            .collect::<Vec<Option<Vec<u64>>>>();
        let mut invalid_list = vec![];
        for mut list in invalids.into_iter().flatten() {
            invalid_list.append(&mut list);
        }
        println!("{invalid_list:?}");
        let s = invalid_list.iter().sum::<u64>();
        println!("{s}");
        Ok(())
    }
}

