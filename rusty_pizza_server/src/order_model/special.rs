#[derive(Debug, PartialEq)]
pub struct SpecialFactory {}

impl SpecialFactory {
    pub fn new() -> SpecialFactory {
        SpecialFactory {}
    }

    pub fn create_special(&mut self, description: String) -> Special {
        Special { id: 0, description }
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

    pub fn get_id(&self) -> u32 {
        self.id
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

        // When:
        let special = special_factory.create_special(String::from("Käserand"));

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
    fn specials_created_through_factory_have_unique_ids() {
        // Given:
        let mut special_factory = SpecialFactory::new();

        // When:
        let special1_id = special_factory.create_special(String::from("Käserand")).get_id();
    }
}
