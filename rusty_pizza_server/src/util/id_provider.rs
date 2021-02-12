pub struct IdProvider {}

impl IdProvider {
    fn new() -> IdProvider {
        IdProvider {}
    }

    fn generate_next(&mut self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_id_is_zero() {
        // Given:
        let mut id_provider = IdProvider::new();

        // When:
        let id = id_provider.generate_next();
    }
}
