use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Money {
    cents: u32,
}

impl Money {
    /// Creates a new `Money` instance from `euros` and `cents`.
    ///
    /// Note that this method does not limit the amount of `cents` to `99`. You can happily pass any amount:
    /// ```
    /// let money = Money::new(1, 205);
    /// assert_eq!(money, Money::new(3, 5));
    /// ```
    pub fn new(euros: u32, cents: u8) -> Money {
        Money {
            cents: euros * 100 + cents as u32,
        }
    }

    pub fn get_euros(&self) -> u32 {
        self.cents / 100
    }

    pub fn get_cents(&self) -> u8 {
        (self.cents % 100) as u8
    }

    pub fn get_total_cents(&self) -> u32 {
        self.cents
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cents: self.cents + other.cents,
        }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            cents: self.cents + other.cents,
        }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            cents: self.cents - other.cents,
        }
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            cents: self.cents - other.cents,
        }
    }
}

impl Mul<u8> for Money {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self {
            cents: self.cents * other as u32,
        }
    }
}

impl Mul<Money> for u8 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
    }
}

impl MulAssign<u8> for Money {
    fn mul_assign(&mut self, other: u8) {
        *self = Self {
            cents: self.cents * other as u32,
        }
    }
}

impl Mul<u16> for Money {
    type Output = Self;

    fn mul(self, other: u16) -> Self {
        Self {
            cents: self.cents * other as u32,
        }
    }
}

impl Mul<Money> for u16 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
    }
}

impl MulAssign<u16> for Money {
    fn mul_assign(&mut self, other: u16) {
        *self = Self {
            cents: self.cents * other as u32,
        }
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Self {
            cents: self.cents * other,
        }
    }
}

impl Mul<Money> for u32 {
    type Output = Money;

    fn mul(self, other: Money) -> Money {
        other * self
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{}€", self.get_euros(), self.get_cents())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fmt::Write;

    #[rstest(euros, cents, expected, case(5, 50, 550), case(7, 20, 720))]
    fn money_can_be_created(euros: u32, cents: u8, expected: u32) {
        // When:
        let money = Money::new(euros, cents);

        // Then:
        assert_eq!(money, Money { cents: expected });
    }

    #[rstest(addend1, addend2, sum,
        case(Money::new(7, 20), Money::new(5, 50), Money { cents: 1270 }),
        case(Money::new(8, 21), Money::new(4, 55), Money { cents: 1276 }),
    )]
    fn money_can_be_summed(addend1: Money, addend2: Money, sum: Money) {
        // When:
        let result = addend1 + addend2;

        // Then:
        assert_eq!(result, sum);
    }

    #[rstest(minuend, subtrahent, difference,
        case(Money::new(7, 20), Money::new(5, 50), Money { cents: 170 }),
        case(Money::new(7, 20), Money::new(5, 55), Money { cents: 165 }),
    )]
    fn money_can_be_subtracted(minuend: Money, subtrahent: Money, difference: Money) {
        // When:
        let result = minuend - subtrahent;

        // Then:
        assert_eq!(result, difference)
    }

    #[test]
    #[should_panic]
    fn creating_negative_amount_of_money_through_subtraction_panics() {
        // When:
        let _ = Money::new(7, 20) - Money::new(7, 40);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u8, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u8, Money { cents: 615 }),
    )]
    fn money_can_be_multiplied_with_u8(money: Money, factor: u8, product: Money) {
        // When:
        let result = money * factor;

        // Then:
        assert_eq!(result, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u8, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u8, Money { cents: 615 }),
    )]
    fn u8_can_be_multiplied_with_money(money: Money, factor: u8, product: Money) {
        // When:
        let result = factor * money;

        // Then:
        assert_eq!(result, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u16, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u16, Money { cents: 615 }),
    )]
    fn money_can_be_multiplied_with_u16(money: Money, factor: u16, product: Money) {
        // When:
        let result = money * factor;

        // Then:
        assert_eq!(result, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u16, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u16, Money { cents: 615 }),
    )]
    fn u16_can_be_multiplied_with_money(money: Money, factor: u16, product: Money) {
        // When:
        let result = factor * money;

        // Then:
        assert_eq!(result, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2, Money { cents: 1000 }),
        case(Money::new(2, 5), 3, Money { cents: 615 }),
    )]
    fn money_can_be_multiplied_with_u32(money: Money, factor: u32, product: Money) {
        // When:
        let result = money * factor;

        // Then:
        assert_eq!(result, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2, Money { cents: 1000 }),
        case(Money::new(2, 5), 3, Money { cents: 615 }),
    )]
    fn u32_can_be_multiplied_with_money(money: Money, factor: u32, product: Money) {
        // When:
        let result = factor * money;

        // Then:
        assert_eq!(result, product);
    }

    #[test]
    fn money_is_copy() {
        // Given:
        let money = Money::new(2, 0);

        // When:
        let result1 = 3u32 * money;
        let result2 = money + Money::new(1, 0);

        // Then:
        assert_eq!(money, Money { cents: 200 });
        assert_eq!(result1, Money { cents: 600 });
        assert_eq!(result2, Money { cents: 300 });
    }

    #[test]
    fn money_prints_as_euro_amount() {
        // Given:
        let money = Money::new(2, 99);

        // When:
        let mut output = String::new();
        write!(&mut output, "{}", money).expect("Error formatting money");

        // Then:
        assert_eq!(output, "2,99€");
    }

    #[rstest(
        money,
        expected,
        case(Money::new(2, 99), 2),
        case(Money::new(3, 99), 3),
        case(Money::new(3, 179), 4)
    )]
    fn can_get_euros_from_money(money: Money, expected: u32) {
        // When:
        let result = money.get_euros();

        // Then:
        assert_eq!(result, expected);
    }

    #[rstest(
        money,
        expected,
        case(Money::new(2, 99), 99),
        case(Money::new(3, 79), 79),
        case(Money::new(3, 179), 79)
    )]
    fn can_get_cents_from_money(money: Money, expected: u8) {
        // When:
        let result = money.get_cents();

        // Then:
        assert_eq!(result, expected);
    }

    #[rstest(
        money,
        expected,
        case(Money::new(2, 99), 299),
        case(Money::new(3, 79), 379),
        case(Money::new(3, 179), 479)
    )]
    fn can_get_total_cents_from_money(money: Money, expected: u32) {
        // When:
        let result = money.get_total_cents();

        // Then:
        assert_eq!(result, expected);
    }

    #[rstest(
        addend1,
        addend2,
        sum,
        case(Money::new(7, 20), Money::new(5, 50), Money { cents: 1270 }),
        case(Money::new(8, 21), Money::new(4, 55), Money { cents: 1276 }),
    )]
    fn money_can_be_add_assigned(mut addend1: Money, addend2: Money, sum: Money) {
        // When:
        addend1 += addend2;

        // Then:
        assert_eq!(addend1, sum);
    }

    #[rstest(minuend, subtrahent, difference,
        case(Money::new(7, 20), Money::new(5, 50), Money { cents: 170 }),
        case(Money::new(7, 20), Money::new(5, 55), Money { cents: 165 }),
    )]
    fn money_can_be_sub_assigned(mut minuend: Money, subtrahent: Money, difference: Money) {
        // When:
        minuend -= subtrahent;

        // Then:
        assert_eq!(minuend, difference)
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u8, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u8, Money { cents: 615 }),
    )]
    fn money_can_be_mul_assigned_with_u8(mut money: Money, factor: u8, product: Money) {
        // When:
        money *= factor;

        // Then:
        assert_eq!(money, product);
    }

    #[rstest(money, factor, product,
        case(Money::new(5, 0), 2u16, Money { cents: 1000 }),
        case(Money::new(2, 5), 3u16, Money { cents: 615 }),
    )]
    fn money_can_be_mul_assigned_with_u16(mut money: Money, factor: u16, product: Money) {
        // When:
        money *= factor;

        // Then:
        assert_eq!(money, product);
    }
}
