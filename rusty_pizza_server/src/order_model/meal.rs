use crate::order_model::special::Special;
use crate::util::id_provider::IdProvider;
use crate::util::money::Money;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct MealFactory {
    id_provider: IdProvider,
}

impl MealFactory {
    pub fn new() -> MealFactory {
        MealFactory {
            id_provider: IdProvider::new(),
        }
    }

    pub fn create_meal(&mut self, meal_id: String, variety: String, price: Money) -> Meal {
        Meal {
            id: self.id_provider.generate_next(),
            meal_id,
            variety,
            price,
            specials: BTreeSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Meal {
    /// Unique ID of this meal
    id: u32,
    /// Number of the meal in the menu
    meal_id: String,
    /// Size of the pizza or noodle type etc.
    variety: String,
    specials: BTreeSet<Special>,
    price: Money,
}

impl Meal {
    pub fn new(id: u32, meal_id: String, variety: String, price: Money) -> Meal {
        Meal {
            id,
            meal_id,
            variety,
            price,
            specials: BTreeSet::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn add_special(&mut self, description: String) -> &Special {
        let special = Special::new(0, description);
        self.specials.insert(special.clone());
        self.specials.get(&special).unwrap()
    }

    pub fn remove_special(&mut self, special: &Special) {
        self.specials.remove(special);
    }

    pub fn get_price(&self) -> Money {
        self.price.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meal_can_be_created() {
        // When:
        let meal = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );

        // Then:
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                specials: BTreeSet::new(),
                price: Money::new(5, 50),
            }
        );
    }

    #[test]
    fn special_can_be_added_to_meal() {
        //Given
        let mut meal = Meal {
            id: 0,
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials: BTreeSet::new(),
        };

        //When
        let special = meal.add_special(String::from("Käserand"));

        //Then
        assert_eq!(special, &Special::new(0, String::from("Käserand")));
        
        let mut expected_specials = BTreeSet::new();
        expected_specials.insert(Special::new(0, String::from("Käserand")));
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: expected_specials,
            }
        );
    }

    #[test]
    fn special_can_be_removed_from_meal() {
        //Given
        let mut specials = BTreeSet::new();
        specials.insert(Special::new(0, String::from("Käserand")));
        let mut meal = Meal {
            id: 0,
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials,
        };

        let special = Special::new(0, String::from("Käserand"));

        //When
        meal.remove_special(&special);

        //Then
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn meal_can_be_created_through_factory() {
        // Given:
        let mut meal_factory = MealFactory::new();

        // When:
        let meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));

        // Then:
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn meals_created_through_factory_have_unique_ids() {
        // Given:
        let mut meal_factory = MealFactory::new();

        // When:
        let meal1_id = meal_factory
            .create_meal(String::from("03"), String::from("groß"), Money::new(5, 50))
            .get_id();
        let meal2_id = meal_factory
            .create_meal(String::from("03"), String::from("groß"), Money::new(5, 50))
            .get_id();

        // Then:
        assert!(meal1_id != meal2_id);
    }
}
