#[derive(Debug, PartialEq, Eq, Hash)]
struct Money {
    cents: u32,
}

impl Money {
    fn new(euros: u32, cents: u8) -> Money {
        Money { cents: euros * 100 + cents as u32 }
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
}