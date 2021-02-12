pub struct IdProvider {}

impl IdProvider {
    fn new() -> IdProvider {
        IdProvider {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_id_is_zero() {
        // Given:
        let id_provider = IdProvider::new();
    }
}
