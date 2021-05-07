use crate::order_model::special::{Special, SpecialFactory};
use crate::util::errors::RemoveError;
use crate::util::id_provider::IdProvider;
use crate::util::money::Money;
use std::collections::HashMap;
use std::iter::Iterator;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BuildPriceError<'a> {
    NegativePriceBuilded(Money, &'a mut MealBuilder), 
}

impl fmt::Display for BuildPriceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BuildPriceError::*;
        match &*self {
            NegativePriceBuilded(negative, _builder) => write!( f, "You have set a negative price: -{:?}", negative),
        }
    }
}

impl Error for BuildPriceError<'_> {}

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
        Meal::new(self.id_provider.generate_next(), meal_id, variety, price)
    }
}

pub struct Specials<'a>(std::collections::hash_map::Values<'a, u32, Special>);

impl<'a> Iterator for Specials<'a> {
    type Item = &'a Special;

    fn next(&mut self) -> Option<&'a Special> {
        self.0.next()
    }
}

pub struct SpecialsMut<'a>(std::collections::hash_map::ValuesMut<'a, u32, Special>);

impl<'a> Iterator for SpecialsMut<'a> {
    type Item = &'a mut Special;

    fn next(&mut self) -> Option<&'a mut Special> {
        self.0.next()
    }
}

#[derive(Debug, PartialEq)]
pub struct MealBuilder {
    /// Number of the meal in the menu
    meal_id: String,
    /// Size of the pizza or noodle type etc.
    variety: Option<String>,
    price: Option<Money>,
    specials: HashMap<u32, Special>,
    special_factory: SpecialFactory,
}

impl MealBuilder {
    pub fn new() -> MealBuilder {
        MealBuilder {
            meal_id: None,
            variety: None,
            price: None,
            specials: HashMap::new(),
            special_factory: SpecialFactory::new(),
        }
    }

    /// Set meal_id of new Meal
    pub fn meal_id<'a>(&'a mut self, meal_id: String) -> &'a mut MealBuilder {
        self.meal_id = Some(meal_id);
        self
    }

    /// Set variety of new Meal
    pub fn variety<'a>(&'a mut self, variety: String) -> &'a mut MealBuilder {
        self.variety = Some(variety);
        self
    }

    /// Add a special to new Meal
    pub fn special<'a>(&'a mut self, description: String) -> &'a mut MealBuilder {
        let special = self.special_factory.create_special(description);
        let id = special.get_id();
        self.specials.insert(id, special);
        self
    }

    /// Add multiple specials to new Meal
    pub fn specials<'a>(&'a mut self, descriptions: &[String]) -> &'a mut MealBuilder {
        let specials = descriptions.iter().
            map(|description| self.special_factory.create_special(description.to_string())).
            map(|special| (special.get_id(), special)).
            collect::<HashMap<u32, Special>>();
        self.specials.extend(specials);
        self
    }

    /// Set total price of new Meal
    pub fn price<'a>(&'a mut self, price: Money) -> &'a mut MealBuilder {
        self.price = Some(price);
        self
    }

    /// Add a new Price to the total price to set the new total price of new Meal
    pub fn add_price<'a>(&'a mut self, price: Money) -> &'a mut MealBuilder {
        self.price = match self.price {
            Some(old) => Some(old + price),
            None => Some(price),
        };
        self
    }

    /// Subtract a new Price from the total price to set the new total price of new Meal
    pub fn diff_price<'a>(&'a mut self, price: Money) -> Result<&'a mut MealBuilder, BuildPriceError> {
        self.price = match self.price {
            Some(old) if old >= price => Some(old - price),
            Some(old) if old < price => return Err(BuildPriceError::NegativePriceBuilded(price - old, self)),
            None => return Err(BuildPriceError::NegativePriceBuilded(price, self)),
            _ => panic!("This should not be possible to reach"),
        };
        Ok(self)
    }

    /// Add a special and its price to new Meal
    pub fn special_with_price<'a>(&'a mut self, description: String, price: Money) -> &'a mut MealBuilder {
        self.special(description).add_price(price)
    }

    /// Add multiple specials and their prices to new Meal
    pub fn specials_with_prices<'a>(&'a mut self, descriptions_prices: &[(String, Money)]) -> &'a mut MealBuilder {
        for (description, price) in descriptions_prices.iter() {
            self.special(description.to_string()).add_price(*price);
        }
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Meal {
    /// Unique ID of this meal
    id: u32,
    /// Number of the meal in the menu
    meal_id: String,
    /// Size of the pizza or noodle type etc.
    variety: String,
    price: Money,
    specials: HashMap<u32, Special>,
    special_factory: SpecialFactory,
}

impl Meal {
    fn new(id: u32, meal_id: String, variety: String, price: Money) -> Meal {
        Meal {
            id,
            meal_id,
            variety,
            price,
            specials: HashMap::new(),
            special_factory: SpecialFactory::new(),
        }
    }

    fn from_builder(id: u32, meal_id: String, variety: String, price: Money, specials: HashMap<u32, Special>, special_factory: SpecialFactory) -> Meal {
        Meal {
            id,
            meal_id,
            variety,
            price,
            specials,
            special_factory,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_price(&self) -> Money {
        self.price
    }

    /// Creates and adds a new special and returns a mutable reference to it.
    pub fn add_special(&mut self, description: String) -> &mut Special {
        let special = self.special_factory.create_special(description);
        let id = special.get_id();
        self.specials.insert(id, special);
        self.specials.get_mut(&id).unwrap()
    }

    pub fn remove_special(&mut self, id: u32) -> Result<Special, RemoveError> {
        self.specials.remove(&id).ok_or(RemoveError::NotFound)
    }

    pub fn specials(&self) -> Specials {
        Specials(self.specials.values())
    }

    pub fn specials_mut(&mut self) -> SpecialsMut {
        SpecialsMut(self.specials.values_mut())
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
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
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
            specials: HashMap::new(),
            special_factory: SpecialFactory::new(),
        };

        //When
        let special = meal.add_special(String::from("Käserand"));

        //Then
        assert_eq!(special, &Special::new(0, String::from("Käserand")));

        let mut expected_special_factory = SpecialFactory::new();
        let mut expected_specials = HashMap::new();
        let expected_special = expected_special_factory.create_special(String::from("Käserand"));
        expected_specials.insert(expected_special.get_id(), expected_special);
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: expected_specials,
                special_factory: expected_special_factory,
            }
        );
    }

    #[test]
    fn added_special_is_mutable() {
        //Given
        let mut meal = Meal {
            id: 0,
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials: HashMap::new(),
            special_factory: SpecialFactory::new(),
        };
        let special = meal.add_special(String::from("Kaserand"));

        //When
        special.set_description(String::from("Käserand"));

        //Then
        assert_eq!(special.get_description(), String::from("Käserand"));
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
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
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

    #[test]
    fn added_specials_have_unique_ids() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));

        // When:
        let special1_id = meal.add_special(String::from("Käserand")).get_id();
        let special2_id = meal.add_special(String::from("Extra scharf")).get_id();

        // Then:
        assert!(special1_id != special2_id);
    }

    #[test]
    fn special_can_be_removed_by_id() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));
        meal.add_special(String::from("Käserand"));

        // When:
        let special = meal.remove_special(0);

        // Then:
        assert_eq!(special, Ok(Special::new(0, String::from("Käserand"))));

        let mut expected_special_factory = SpecialFactory::new();
        expected_special_factory.create_special(String::from("Käserand"));
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: expected_special_factory,
            }
        )
    }

    #[test]
    fn remving_not_existing_special_returns_not_found() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));

        // When:
        let special = meal.remove_special(0);

        // Then:
        assert_eq!(special, Err(RemoveError::NotFound));
        assert_eq!(
            meal,
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            }
        )
    }

    #[test]
    fn specials_can_be_iterated() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));
        meal.add_special(String::from("Käserand"));

        // When:
        let mut specials = meal.specials();

        // Then:
        assert_eq!(
            specials.next(),
            Some(&Special::new(0, String::from("Käserand")))
        );
        assert_eq!(specials.next(), None);
    }

    #[test]
    fn specials_can_be_mutably_iterated() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal =
            meal_factory.create_meal(String::from("03"), String::from("groß"), Money::new(5, 50));
        meal.add_special(String::from("Käserand"));

        // When:
        let mut specials = meal.specials_mut();

        // Then:
        assert_eq!(
            specials.next(),
            Some(&mut Special::new(0, String::from("Käserand")))
        );
        assert_eq!(specials.next(), None);
    }
}
