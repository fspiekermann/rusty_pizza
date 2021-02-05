struct Money {

}

impl Money {
    fn new(before_decimal_point: u32, after_decimal_point: u8) -> Money {
        Money { }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_can_be_created() {
        // When:
        let money = Money::new(5, 50);
    }
}