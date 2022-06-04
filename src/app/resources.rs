use std::{fs::File, collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_json;



#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]

pub enum Besoldung {
    A3 = 2476,
    A4 = 2553,
    A5 = 2598,
    A6 = 2700,
    A7 = 2864,
    A8 = 3090,
    A9 = 3333,
    A10 = 3654,
    A11 = 4192,
    A12 = 4565,
    A13 = 5193,
    A14 = 5508,
    A15 = 6330,
    A16 = 7018
}

impl Default for Besoldung {
    fn default() -> Self {
        Self::A5
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, Clone, PartialEq)]

pub struct Soldat {
    pub besoldung: Besoldung
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct Unit {
    pub name: String,
    pub kaufpreis: i64,
    pub url: String,
    pub soldaten: usize,
    pub wartung_pro_jahr: i64,
}

impl Unit {
    pub fn total_cost(&self) -> i64 {
        self.kaufpreis + self.soldaten as i64 * 3000 * 12 + self.wartung_pro_jahr
    }
}

#[test]
fn write_unit() {

    let mut leopard2 = Unit::default();
    // leopard2.soldaten.push(Soldat::default());
    serde_json::to_writer_pretty(File::create("leo.json").unwrap(), &leopard2).unwrap();
}