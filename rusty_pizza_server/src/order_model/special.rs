#[derive(Debug, PartialEq)]
pub struct SpecialFactory {

}

impl SpecialFactory {
    pub fn new() -> SpecialFactory {
        SpecialFactory { }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Special {
    id: u32,
    description: String,
}

impl Special {
    pub fn new(id: u32, description: String) -> Special {
        Special { id, description }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_can_be_created() {
        // When:
        let special = Special::new(0, String::from("Käserand"));

        // Then:
        assert_eq!(
            special,
            Special {
                id: 0,
                description: String::from("Käserand")
            }
        );
    }

    #[test]
    fn special_can_be_created_through_factory() {
        // Given:
        let mut special_factory = SpecialFactory::new();
    }
}
