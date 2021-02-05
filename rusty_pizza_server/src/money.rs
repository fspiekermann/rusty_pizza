use std::ops::{Add, Sub};

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
        // Given:
        let money1 = Money::new(7, 20);
        let money2 = Money::new(5, 50);

        // When:
        let result = money1 + money2;

        // Then:
        assert_eq!(result, Money { cents: 1270 })
    }

    #[test]
    fn money_can_be_added_with_different_values() {
        // Given:
        let money1 = Money::new(7, 20);
        let money2 = Money::new(5, 55);

        // When:
        let result = money1 + money2;

        // Then:
        assert_eq!(result, Money { cents: 1275 })
    }

    #[test]
    fn money_can_be_subtracted() {
        // Given:
        let money1 = Money::new(7, 20);
        let money2 = Money::new(5, 50);

        // When:
        let result = money1 - money2;

        // Then:
        assert_eq!(result, Money { cents: 170 })
    }

    #[test]
    fn money_can_be_subtracted_with_different_values() {
        // Given:
        let money1 = Money::new(7, 20);
        let money2 = Money::new(5, 55);

        // When:
        let result = money1 - money2;

        // Then:
        assert_eq!(result, Money { cents: 165 })
    }

    #[test]
    #[should_panic]
    fn creating_negative_amount_of_money_through_subtraction_panics() {
        // Given:
        let money1 = Money::new(7, 20);
        let money2 = Money::new(7, 40);

        // When:
        let _ = money1 - money2;
    }
}