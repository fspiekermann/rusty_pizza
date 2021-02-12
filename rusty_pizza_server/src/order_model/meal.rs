use std::collections::BTreeSet;
use crate::util::money::Money;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Meal {
    /// Number of the meal in the menu
    meal_id: String,
    /// Size of the pizza or noodle type etc.
    variety: String,
    specials: BTreeSet<String>,
    price: Money,
}

impl Meal {
    fn add_special(&mut self, special: String) {
        self.specials.insert(special);
    }

    fn remove_special(&mut self, special: &String) {
        self.specials.remove(special);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_can_be_added_to_meal() {
        //Given
        let mut meal = Meal {
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials: BTreeSet::new(),
        };

        let special = String::from("Käserand");

        //When
        meal.add_special(special);

        //Then
        let mut expected_specials = BTreeSet::new();
        expected_specials.insert(String::from("Käserand"));
        assert_eq!(
            meal,
            Meal {
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
        specials.insert(String::from("Käserand"));
        let mut meal = Meal {
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials,
        };

        let special = String::from("Käserand");

        //When
        meal.remove_special(&special);

        //Then
        assert_eq!(
            meal,
            Meal {
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: BTreeSet::new(),
            }
        );
    }

}