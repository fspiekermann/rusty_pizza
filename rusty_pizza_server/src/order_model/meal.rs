use crate::order_model::special::{Special, SpecialFactory};
use crate::util::errors::RemoveError;
use crate::util::id_provider::IdProvider;
use crate::util::money::Money;
use std::collections::HashMap;
use std::iter::Iterator;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MealBuilderError {
    NegativePriceBuilded(Money),
    MoreSpecialsThanPrices(usize),
    MorePricesThanSpecials(usize),
}

impl fmt::Display for MealBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MealBuilderError::*;
        match &*self {
            NegativePriceBuilded(negative_amount) => write!( f, "You have set a negative price: -{:?}", negative_amount),
            MoreSpecialsThanPrices(more_quantity) => write!( f, "You gave {:?} more specials than prices!", more_quantity),
            MorePricesThanSpecials(more_quantity) => write!( f, "You gave {:?} more prices than specials!", more_quantity),
        }
    }
}

impl Error for MealBuilderError {}

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

    pub fn start_by(starting_value: u32) -> MealFactory {
        MealFactory {
            id_provider: IdProvider::start_by(starting_value),
        }
    }

    pub fn create_meal(&mut self, meal_id: String, variety: String, price: Money) -> Meal {
        Meal::new(self.id_provider.generate_next(), meal_id, variety, price, HashMap::new(), SpecialFactory::new())
    }

    pub fn create_meal_with_specials(&mut self, meal_id: String, variety: String, price: Money, specials: HashMap<u32, Special>, special_factory: SpecialFactory,) -> Meal {
        Meal::new(self.id_provider.generate_next(), meal_id, variety, price, specials, special_factory)
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
    meal_id: Option<String>,
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
    pub fn diff_price<'a>(&'a mut self, price: Money) -> Result<&'a mut MealBuilder, MealBuilderError> {
        self.price = match self.price {
            Some(old) if old >= price => Some(old - price),
            Some(old) if old < price => return Err(MealBuilderError::NegativePriceBuilded(price - old)),
            None => return Err(MealBuilderError::NegativePriceBuilded(price)),
            _ => panic!("This should not be possible to reach"),
        };
        Ok(self)
    }

    /// Add a special and its price to new Meal
    pub fn special_with_price<'a>(&'a mut self, description: String, price: Money) -> &'a mut MealBuilder {
        self.special(description).add_price(price)
    }

    /// Add multiple specials and their prices to new Meal
    pub fn specials_with_prices<'a>(&'a mut self, descriptions: &[String], prices: &[Money]) -> Result<&'a mut MealBuilder, MealBuilderError> {
        if descriptions.len() > prices.len() {
            return Err(MealBuilderError::MoreSpecialsThanPrices(descriptions.len() - prices.len()))
        }
        if prices.len() > descriptions.len() {
            return Err(MealBuilderError::MorePricesThanSpecials(prices.len() - descriptions.len()))
        }
        for (description, price) in descriptions.iter().zip(prices.iter()) {
            self.special(description.to_string()).add_price(*price);
        }
        Ok(self)
    }

    pub fn meal(self, meal_factory: &mut MealFactory) -> Meal {
        meal_factory.create_meal_with_specials(
            self.meal_id.unwrap_or(String::from("")),
            self.variety.unwrap_or(String::from("")),
            self.price.unwrap_or(Money::new(0,0)),
            self.specials,
            self.special_factory,
        )
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
    fn new(id: u32, meal_id: String, variety: String, price: Money, specials: HashMap<u32, Special>, special_factory: SpecialFactory) -> Meal {
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

    pub fn get_meal_id(&self) -> String {
        self.meal_id.clone()
    }

    pub fn get_variety(&self) -> String {
        self.variety.clone()
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
    use rstest::*;
    use crate::order_model::meal::MealBuilderError::*;

    #[test]
    fn meal_can_be_created() {
        // When:
        let meal = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
            HashMap::new(),
            SpecialFactory::new(),
        );

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            },
            meal,
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
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: expected_specials,
                special_factory: expected_special_factory,
            },
            meal,
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
        assert_eq!(String::from("Käserand"), special.get_description());
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
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            },
            meal,
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
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: expected_special_factory,
            },
            meal,
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
            Meal {
                id: 0,
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            },
            meal,
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
            Some(&Special::new(0, String::from("Käserand"))),
            specials.next(),
        );
        assert_eq!(None, specials.next());
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
            Some(&mut Special::new(0, String::from("Käserand"))),
            specials.next(),
        );
        assert_eq!(None, specials.next());
    }

    #[test]
    fn meal_builder_create_meal_with_default_values() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let meal_builder = MealBuilder::new();
        
        // When:
        let meal = meal_builder.meal(&mut meal_factory);

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from(""),
                variety: String::from(""),
                price: Money::new(0, 0),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            },
            meal,
        )
    }

    #[test]
    fn meal_builder_create_meal_without_spacials() {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        
        // When:
        meal_builder.
            meal_id(String::from("05")).
            variety(String::from("Big")).
            price(Money::new(4, 50));
        let meal = meal_builder.meal(&mut meal_factory);

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("05"),
                variety: String::from("Big"),
                price: Money::new(4, 50),
                specials: HashMap::new(),
                special_factory: SpecialFactory::new(),
            },
            meal,
        )
    }

    #[rstest]
    #[case(vec![String::from("Pan-Pizza"), String::from("Zwiebeln"), String::from("Knoblauch")])]
    fn single_specials_can_be_added_to_meal_builder(
        #[case] specials: Vec<String>,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        let mut expected_special_factory = SpecialFactory::new();
        let mut expected_specials = HashMap::new();
        
        // When:
        meal_builder.
            meal_id(String::from("05")).
            variety(String::from("Big")).
            price(Money::new(4, 50));
        for new_special in specials.into_iter() {
            meal_builder.special(new_special.clone());
            let special = expected_special_factory.create_special(new_special);
            let id = special.get_id();
            expected_specials.insert(id, special);
        
        }
        let meal = meal_builder.meal(&mut meal_factory);

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("05"),
                variety: String::from("Big"),
                price: Money::new(4, 50),
                specials: expected_specials,
                special_factory: expected_special_factory,
            },
            meal,
        )
    }
    
    #[rstest]
    #[case(vec![String::from("Pan-Pizza"), String::from("Zwiebeln"), String::from("Knoblauch")])]
    fn bundle_of_specials_can_be_added_to_meal_builder(
        #[case] specials: Vec<String>,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        let mut expected_special_factory = SpecialFactory::new();
        let mut expected_specials = HashMap::new();
        
        // When:
        meal_builder.
            meal_id(String::from("05")).
            variety(String::from("Big")).
            price(Money::new(4, 50)).
            specials(&specials);
        let meal = meal_builder.meal(&mut meal_factory);
        for new_special in specials.into_iter() {
            let special = expected_special_factory.create_special(new_special);
            let id = special.get_id();
            expected_specials.insert(id, special);
        
        }

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("05"),
                variety: String::from("Big"),
                price: Money::new(4, 50),
                specials: expected_specials,
                special_factory: expected_special_factory,
            },
            meal,
        )
    }

    
    #[rstest]
    #[case(vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)], Money::new(12, 26))]
    #[case(vec![Money::new(7, 50), Money::new(1, 42)], Money::new(8, 92))]
    #[case(vec![Money::new(3, 33)], Money::new(3, 33))]
    fn prices_are_summed_up_correctly_in_mealbuilder(
        #[case] prices: Vec<Money>,
        #[case] expected_sum: Money,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();

        // When:
        for price in prices.into_iter() {
            meal_builder.add_price(price);
        }
        let summed_price = meal_builder.meal(&mut meal_factory).get_price();

        // Then:
        assert_eq!(expected_sum, summed_price)
    }

    
    #[rstest]
    #[case(Money::new(25, 00), vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)], Money::new(12, 74))]
    #[case(Money::new(25, 00), vec![Money::new(7, 50), Money::new(1, 42)], Money::new(16, 08))]
    #[case(Money::new(25, 00), vec![Money::new(3, 33)], Money::new(21, 67))]
    fn prices_are_subtracted_up_correctly_in_mealbuilder(
        #[case] start_value: Money,
        #[case] prices: Vec<Money>,
        #[case] expected_diff: Money,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        meal_builder.price(start_value);

        // When:
        for price in prices.into_iter() {
            meal_builder.diff_price(price).unwrap();
        }
        let subtracted_price = meal_builder.meal(&mut meal_factory).get_price();

        // Then:
        assert_eq!(expected_diff, subtracted_price)
    }

    
    #[rstest]
    #[case(Money::new(10, 00), vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)], NegativePriceBuilded(Money::new(2, 26)))]
    #[case(Money::new(5, 00), vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)], NegativePriceBuilded(Money::new(2, 43)))]
    #[case(Money::new(7, 50), vec![Money::new(7, 50), Money::new(1, 42)], NegativePriceBuilded(Money::new(1, 42)))]
    #[case(Money::new(1, 00), vec![Money::new(3, 33)], NegativePriceBuilded(Money::new(2, 33)))]
    fn negative_price_substraction_returns_error_in_mealbuilder(
        #[case] start_value: Money,
        #[case] prices: Vec<Money>,
        #[case] expected_negativ: MealBuilderError,
    ) {
        // Given:
        let mut meal_builder = MealBuilder::new();
        meal_builder.price(start_value);
        let mut negative_amount = NegativePriceBuilded(Money::new(0, 0));

        // When:
        for price in prices.into_iter() {
            match meal_builder.diff_price(price) {
                Ok(_) => continue,
                Err(negative_error) => {
                    negative_amount = negative_error;
                    break;
                }
            }
        }

        // Then:
        assert_eq!(expected_negativ, negative_amount)
    }

    #[rstest]
    #[case(
        vec![String::from("Pan-Pizza"), String::from("Zwiebeln"), String::from("Knoblauch")],
        vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)],
        Money::new(12, 26)
    )]
    fn single_specials_with_price_can_be_added_to_meal_builder(
        #[case] specials: Vec<String>,
        #[case] prices: Vec<Money>,
        #[case] expected_sum: Money,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        let mut expected_special_factory = SpecialFactory::new();
        let mut expected_specials = HashMap::new();
        
        // When:
        meal_builder.
            meal_id(String::from("05")).
            variety(String::from("Big"));
        for (new_special, new_price) in specials.into_iter().zip(prices.into_iter()) {
            meal_builder.special_with_price(new_special.clone(), new_price);
            let special = expected_special_factory.create_special(new_special);
            let id = special.get_id();
            expected_specials.insert(id, special);
        
        }
        let meal = meal_builder.meal(&mut meal_factory);

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("05"),
                variety: String::from("Big"),
                price: expected_sum,
                specials: expected_specials,
                special_factory: expected_special_factory,
            },
            meal,
        )
    }
    
    #[rstest]
    #[case(
        vec![String::from("Pan-Pizza"), String::from("Zwiebeln"), String::from("Knoblauch")],
        vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)],
        Money::new(12, 26)
    )]
    fn bundle_of_specials_with_prices_can_be_added_to_meal_builder(
        #[case] specials: Vec<String>,
        #[case] prices: Vec<Money>,
        #[case] expected_sum: Money,
    ) {
        // Given:
        let mut meal_factory = MealFactory::new();
        let mut meal_builder = MealBuilder::new();
        let mut expected_special_factory = SpecialFactory::new();
        let mut expected_specials = HashMap::new();
        
        // When:
        meal_builder.
            meal_id(String::from("05")).
            variety(String::from("Big")).
            specials_with_prices(&specials, &prices).unwrap();
        let meal = meal_builder.meal(&mut meal_factory);
        for new_special in specials.into_iter() {
            let special = expected_special_factory.create_special(new_special);
            let id = special.get_id();
            expected_specials.insert(id, special);
        
        }

        // Then:
        assert_eq!(
            Meal {
                id: 0,
                meal_id: String::from("05"),
                variety: String::from("Big"),
                price: expected_sum,
                specials: expected_specials,
                special_factory: expected_special_factory,
            },
            meal,
        )
    }
    
    #[rstest]
    #[case(
        vec![String::from("Pan-Pizza"), String::from("Zwiebeln"), String::from("Knoblauch")],
        vec![Money::new(2, 42), Money::new(5, 01)],
        MoreSpecialsThanPrices(1)
    )]
    #[case(
        vec![String::from("Pan-Pizza")],
        vec![Money::new(2, 42), Money::new(5, 01), Money::new(4, 83)],
        MorePricesThanSpecials(2)
    )]
    fn different_count_of_specials_and_prices_produce_error(
        #[case] specials: Vec<String>,
        #[case] prices: Vec<Money>,
        #[case] expected_err: MealBuilderError,
    ) {
        // Given:
        let mut meal_builder = MealBuilder::new();
        
        // When:
        let meal_err = meal_builder.specials_with_prices(&specials, &prices);

        // Then:
        assert_eq!(
            Err(expected_err),
            meal_err,
        )
    }
}
