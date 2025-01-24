use std::collections::HashMap;

use anyhow::Result;
use rust_decimal::Decimal;

use super::data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct DataDigital {
    // index: f32, value: f32
    datas: HashMap<Decimal, Decimal>,
}
impl Data for DataDigital {
    fn y(&self, x: &Decimal) -> Option<Decimal> {
        let y = self.datas.get(&x);
        y.copied()
    }
}

impl DataDigital {
    pub fn new(datas: Vec<(Decimal, Decimal)>) -> Self {
        let mut content = HashMap::new();
        for data in datas {
            content.insert(data.0, data.1);
        }
        DataDigital { datas: content }
    }
    pub fn build_from_csv(csv: &str) -> Result<Self> {
        let mut datas: HashMap<Decimal, Decimal> = HashMap::new();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv.as_bytes());
        for result in rdr.records() {
            let record = result?;
            let freq = record[0].trim();
            let value = record[1].trim();
            datas.insert(
                Decimal::try_from(freq).unwrap(),
                Decimal::try_from(value).unwrap(),
            );
        }
        Ok(DataDigital { datas })
    }
    pub fn get_data(&self) -> Vec<(Decimal, Decimal)> {
        let mut data: Vec<(Decimal, Decimal)> = self.datas.iter().map(|(&x, &y)| (x, y)).collect();
        data.sort();
        data
    }
    pub fn get_x_list(&self) -> Vec<Decimal> {
        let mut x_list: Vec<Decimal> = self.datas.keys().cloned().collect();
        x_list.sort();
        x_list
    }
    pub fn get_y_list(&self) -> Vec<Decimal> {
        let mut datas: Vec<(Decimal, Decimal)> = self.datas.iter().map(|(x, y)| (*x, *y)).collect();
        datas.sort();
        datas.iter().map(|(_, y)| *y).collect()
    }
    pub fn get(&self, index: usize) -> Option<(Decimal, Decimal)> {
        let mut datas: Vec<(Decimal, Decimal)> = self.datas.iter().map(|(&a, &b)| (a, b)).collect();
        datas.sort();
        datas.get(index).cloned()
    }
    pub fn get_data_count(&self) -> usize {
        self.datas.len()
    }
    pub fn add(&mut self, x: Decimal, y: Decimal) {
        self.datas.insert(x, y);
    }
    pub fn to_csv(&self) -> String {
        let datas = self.datas.clone();
        let mut datas: Vec<(&Decimal, &Decimal)> = datas.iter().collect();
        datas.sort();
        datas
            .iter()
            .map(|(x, y)| format!("{},{}", x, y))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_build_from_csv() {
        let input = "\
        23.0,1329.0
        24.0,2329.0
        25.0,3329.0
        26.0,4329.0";

        let expected = DataDigital::new(vec![
            (
                Decimal::try_from("23.0").unwrap(),
                Decimal::try_from("1329.0").unwrap(),
            ),
            (
                Decimal::try_from("24.0").unwrap(),
                Decimal::try_from("2329.0").unwrap(),
            ),
            (
                Decimal::try_from("25.0").unwrap(),
                Decimal::try_from("3329.0").unwrap(),
            ),
            (
                Decimal::try_from("26.0").unwrap(),
                Decimal::try_from("4329.0").unwrap(),
            ),
        ]);

        let output = DataDigital::build_from_csv(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_2_build_from_csv() {
        let input = "\
        27.0,5329.0
        28.0,6329.0
        29.0,7329.0
        30.0,8329.0";

        let expected = DataDigital::new(vec![
            (
                Decimal::try_from("27.0").unwrap(),
                Decimal::try_from("5329.0").unwrap(),
            ),
            (
                Decimal::try_from("28.0").unwrap(),
                Decimal::try_from("6329.0").unwrap(),
            ),
            (
                Decimal::try_from("29.0").unwrap(),
                Decimal::try_from("7329.0").unwrap(),
            ),
            (
                Decimal::try_from("30.0").unwrap(),
                Decimal::try_from("8329.0").unwrap(),
            ),
        ]);

        let output = DataDigital::build_from_csv(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_3_build_from_csv() {
        let input = "\
        31.0,9329.0
        32.0,10329.0
        33.0,11329.0
        34.0,12329.0";

        let expected = DataDigital::new(vec![
            (
                Decimal::try_from("31.0").unwrap(),
                Decimal::try_from("9329.0").unwrap(),
            ),
            (
                Decimal::try_from("32.0").unwrap(),
                Decimal::try_from("10329.0").unwrap(),
            ),
            (
                Decimal::try_from("33.0").unwrap(),
                Decimal::try_from("11329.0").unwrap(),
            ),
            (
                Decimal::try_from("34.0").unwrap(),
                Decimal::try_from("12329.0").unwrap(),
            ),
        ]);

        let output = DataDigital::build_from_csv(input).unwrap();
        assert_eq!(output, expected);
    }
}
