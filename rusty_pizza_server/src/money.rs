use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Money {
    cents: u32,
}

impl Money {
    fn new(euros: u32, cents: u8) -> Money {
        Money { cents: euros * 100 + cents as u32 }
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { cents: self.cents + other.cents }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { cents: self.cents - other.cents }
    }
}

impl Mul<u8> for Money {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self { cents: self.cents * other as u32 }
    }
}

impl Mul<Money> for u8 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
    }
}

impl Mul<u16> for Money {
    type Output = Self;

    fn mul(self, other: u16) -> Self {
        Self { cents: self.cents * other as u32 }
    }
}

impl Mul<Money> for u16 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Self { cents: self.cents * other }
    }
}

impl Mul<Money> for u32 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
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
        assert_eq!(money, Money { cents: 550 })
    }

    #[test]
    fn money_can_be_created_with_alternative_values() {
        // When:
        let money = Money::new(7, 20);

        // Then:
        assert_eq!(money, Money { cents: 720 })
    }

    #[test]
    fn money_can_be_added() {
        // When:
        let result = Money::new(7, 20) + Money::new(5, 50);

        // Then:
        assert_eq!(result, Money { cents: 1270 })
    }

    #[test]
    fn money_can_be_added_with_different_values() {
        // When:
        let result = Money::new(7, 20) + Money::new(5, 55);

        // Then:
        assert_eq!(result, Money { cents: 1275 })
    }

    #[test]
    fn money_can_be_subtracted() {
        // When:
        let result = Money::new(7, 20) - Money::new(5, 50);

        // Then:
        assert_eq!(result, Money { cents: 170 })
    }

    #[test]
    fn money_can_be_subtracted_with_different_values() {
        // When:
        let result = Money::new(7, 20) - Money::new(5, 55);

        // Then:
        assert_eq!(result, Money { cents: 165 })
    }

    #[test]
    #[should_panic]
    fn creating_negative_amount_of_money_through_subtraction_panics() {
        // When:
        let _ = Money::new(7, 20) - Money::new(7, 40);
    }

    #[test]
    fn money_can_be_multiplied_with_an_u8() {
        // When:
        let result = Money::new(5, 0) * 2u8;

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn money_can_be_multiplied_with_an_u8_with_different_values() {
        // When:
        let result = Money::new(2, 0) * 3u8;

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }

    #[test]
    fn u8_can_be_multiplied_with_money() {
        // When:
        let result = 2u8 * Money::new(5, 0);

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn u8_can_be_multiplied_with_money_with_different_values() {
        // When:
        let result = 3u8 * Money::new(2, 0);

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }

    #[test]
    fn money_can_be_multiplied_with_an_u16() {
        // When:
        let result = Money::new(5, 0) * 2u16;

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn money_can_be_multiplied_with_an_u16_with_different_values() {
        // When:
        let result = Money::new(2, 0) * 3u16;

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }

    #[test]
    fn u16_can_be_multiplied_with_money() {
        // When:
        let result = 2u16 * Money::new(5, 0);

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn u16_can_be_multiplied_with_money_with_different_values() {
        // When:
        let result = 3u16 * Money::new(2, 0);

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }

    #[test]
    fn money_can_be_multiplied_with_an_u32() {
        // When:
        let result = Money::new(5, 0) * 2u32;

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn money_can_be_multiplied_with_an_u32_with_different_values() {
        // When:
        let result = Money::new(2, 0) * 3u32;

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }

    #[test]
    fn u32_can_be_multiplied_with_money() {
        // When:
        let result = 2u32 * Money::new(5, 0);

        // Then:
        assert_eq!(result, Money { cents: 1000 })
    }

    #[test]
    fn u32_can_be_multiplied_with_money_with_different_values() {
        // When:
        let result = 3u32 * Money::new(2, 0);

        // Then:
        assert_eq!(result, Money { cents: 600 })
    }
}