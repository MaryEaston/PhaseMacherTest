pub mod data;
pub mod data_analog;
pub mod data_digital;
pub mod number;

use std::collections::HashMap;

use anyhow::Result;
// use decimal::d128;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use data::Data;

pub fn compare<D1, D2, W>(
    data1: &D1,
    data2: &D2,
    weight: &W,
    compare_xs: &Vec<Decimal>,
) -> Result<Decimal>
where
    D1: Data,
    D2: Data,
    W: Data,
{
    let mut score: Decimal = dec!(0.0);
    for compare_x in compare_xs {
        // log::info!("x:{:?}", compare_x);
        let y1 = data1.y(compare_x).unwrap();
        let y2 = data2.y(compare_x).unwrap();
        let w = weight.y(compare_x).unwrap();

        score += comparison_function(y1, y2, w);
    }
    Ok(score)
}

pub fn search_closest<R, T, W>(
    expected: HashMap<String, R>,
    target: HashMap<String, T>,
    weight: W,
    compare_xs: Vec<Decimal>,
) -> Result<HashMap<(String, String), T>>
where
    R: Data,
    T: Data + Clone,
    W: Data,
{
    let mut result: HashMap<(String, String), T> = HashMap::new();
    for (id_expected, data_expected) in expected.iter() {
        let mut score_list: HashMap<&str, Decimal> = HashMap::new();
        for (id_target, data_target) in target.iter() {
            let score = compare(data_expected, data_target, &weight, &compare_xs)?;
            score_list.insert(id_target, score);
        }
        let (&min_id_target, _score) = score_list.iter().min_by_key(|&(_, v)| v).unwrap();
        let foo = target.get(min_id_target).unwrap().clone();
        result.insert((id_expected.to_string(), min_id_target.to_string()), foo);
    }
    Ok(result)
}

fn comparison_function(value1: Decimal, value2: Decimal, weight: Decimal) -> Decimal {
    phase_difference(value1, value2).powu(2) * weight
}

pub fn phase_difference(angle1: Decimal, angle2: Decimal) -> Decimal {
    // 角度を360度の範囲に正規化 (-360 ~ 360 もカバー)
    let normalize_angle = |angle: f64| -> f64 {
        let mut normalized = angle % 360.0;
        if normalized < 0.0 {
            normalized += 360.0;
        }
        normalized
    };

    // 入力角度を正規化
    let angle1 = normalize_angle(angle1.try_into().unwrap());
    let angle2 = normalize_angle(angle2.try_into().unwrap());

    // 差を計算して360度の範囲に収める
    let mut diff = (angle1 - angle2).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }

    Decimal::from_f64_retain(diff).unwrap()
}
