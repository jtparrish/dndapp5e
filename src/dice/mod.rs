// use lazy_static::lazy_static;
use std::str::FromStr;
use rand::RngCore;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct DiceRoller(StdRng);

enum DiceRollComp {
    Roll { num: u64, val: u64 },
    Modifier(u64),
}

pub struct DiceRoll(Vec<DiceRollComp>);

impl FromStr for DiceRollComp {
    // TODO: probably want a different error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let sub_items: Vec<&str> = s.split("d").collect();
        match sub_items.len() {
            1 => Ok(Self::Modifier( u64::from_str(sub_items[0]).unwrap() )),
            2 => {
                let num = u64::from_str(sub_items[0]).unwrap();
                let val = u64::from_str(sub_items[1]).unwrap();

                Ok(Self::Roll { num: num, val: val})
            },
            _ => return Err(()),
        }
    }
}

impl FromStr for DiceRoll {
    // TODO: probably want a different error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut dr = DiceRoll(Vec::new());
        for item in s.split("+") {
            dr.0.push(DiceRollComp::from_str(item.trim())?);
        }

        Ok(dr)
    }
}

impl Display for DiceRollComp {
    
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DiceRollComp::*;

        match self {
            Roll { num, val } => write!(f, "{}d{}", num, val),
            Modifier(val) => write!(f, "{}", val),
        }
    }
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for item in self.0.iter() {
            if !first {
                write!(f, " + ")?;
            }
            item.fmt(f);
            first = false;
        }

        Ok(())
    }
}

impl Default for DiceRoller {
    fn default() -> Self {
        DiceRoller( StdRng::from_seed(<StdRng as SeedableRng>::Seed::default()) )
    }
}

impl DiceRoller {
    pub fn roll(&mut self, roll: &DiceRoll) -> u64 {
        use DiceRollComp::*;

        let mut total: u64 = 0;
        for item in roll.0.iter() {
            total += match item {
                &Roll { num, val } => {
                    let mut sub_total = 0;
                    for i in 0..num { 
                        sub_total += self.0.next_u64() % val + 1;
                    }

                    sub_total
                },
                &Modifier(val) => val, 
            }
        }

        total
    }
}

impl DiceRoll {
    pub fn avg_roll(&self) -> u64 {
        use DiceRollComp::*;
        let mut total: u64 = 0;
        for item in self.0.iter() {
            total += match *item {
                Roll { num, val } => num * (val + 1) / 2,
                Modifier(val) => val, 
            }
        }

        total
    }
}