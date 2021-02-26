#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IdProvider {
    next_id: u32,
}

impl IdProvider {
    pub fn new() -> IdProvider {
        IdProvider { next_id: 0 }
    }

    pub fn generate_next(&mut self) -> u32 {
        let next = self.next_id;
        self.next_id = next + 1;
        next
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

        // Then:
        assert_eq!(id, 0);
    }

    #[test]
    fn second_id_is_one() {
        // Given:
        let mut id_provider = IdProvider::new();
        id_provider.generate_next();

        // When:
        let id = id_provider.generate_next();

        // Then:
        assert_eq!(id, 1);
    }
}
