use std::sync::Arc;
use std::sync::Mutex;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::data::Data;

#[derive(Clone)]
pub struct DataAnalog {
    // fn(x: f32) -> y: f32
    datas: Arc<Mutex<Box<dyn Fn(&Decimal) -> Option<Decimal>>>>,
}
impl Data for DataAnalog {
    fn y(&self, x: &Decimal) -> Option<Decimal> {
        (self.datas.lock().unwrap())(x)
    }
}
impl DataAnalog {
    pub fn get_line(point1: (Decimal, Decimal), point2: (Decimal, Decimal)) -> Self {
        let (x1, y1) = point1;
        let (x2, y2) = point2;

        // 直線の傾きを計算
        let slope = if x1 == x2 {
            return DataAnalog {
                datas: Arc::new(Mutex::new(Box::new(move |_| None))), // x1 == x2 の場合、垂直線なので None を返す
            };
        } else {
            (y2 - y1) / (x2 - x1)
        };

        // y切片を計算
        let intercept = y1 - slope * x1;

        DataAnalog {
            datas: Arc::new(Mutex::new(Box::new(move |x: &Decimal| {
                Some(slope * *x + intercept)
            }))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_get_line() {
        let point1 = (dec!(1.0), dec!(2.0));
        let point2 = (dec!(3.0), dec!(4.0));

        let line = DataAnalog::get_line(point1, point2);

        assert_eq!(line.y(&dec!(2.0)), Some(dec!(3.0)));
        assert_eq!(line.y(&dec!(4.0)), Some(dec!(5.0)));
    }
}
