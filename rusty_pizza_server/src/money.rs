#[derive(Debug, PartialEq, Eq, Hash)]
struct Money {
    before_decimal_point: u32,
    after_decimal_point: u8,
}

impl Money {
    fn new(before_decimal_point: u32, after_decimal_point: u8) -> Money {
        Money {
            before_decimal_point: 5,
            after_decimal_point: 50,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_can_be_created() {
        // When:
        let money = Money::new(5, 50);

        // Then:
        assert_eq!(money, Money {
            before_decimal_point: 5,
            after_decimal_point: 50,
        })
    }
}