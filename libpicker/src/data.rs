use rust_decimal::Decimal;

pub trait Data {
    fn y(&self, x: &Decimal) -> Option<Decimal>;
}
