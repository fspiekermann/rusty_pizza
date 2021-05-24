use crate::util::id::Id;
use crate::util::id_provider::IdProvider;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SpecialFactory {
    id_provider: IdProvider,
}

impl SpecialFactory {
    pub fn new() -> SpecialFactory {
        SpecialFactory {
            id_provider: IdProvider::new(),
        }
    }

    pub fn create_special(&mut self, description: String) -> Special {
        Special::new(self.id_provider.generate_next(), description)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Special {
    id: Id,
    description: String,
}

impl Special {
    pub fn new(id: Id, description: String) -> Special {
        Special { id, description }
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_can_be_created() {
        // When:
        let special = Special::new(Id::new(0), String::from("Käserand"));

        // Then:
        assert_eq!(
            special,
            Special {
                id: Id::new(0),
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
                id: Id::new(0),
                description: String::from("Käserand")
            }
        );
    }

    #[test]
    fn specials_created_through_factory_have_unique_ids() {
        // Given:
        let mut special_factory = SpecialFactory::new();

        // When:
        let special1_id = special_factory
            .create_special(String::from("Käserand"))
            .get_id();
        let special2_id = special_factory
            .create_special(String::from("Käserand"))
            .get_id();

        // Then:
        assert!(special1_id != special2_id);
    }
}
