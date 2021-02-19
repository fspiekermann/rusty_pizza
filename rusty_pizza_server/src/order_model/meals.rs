use crate::order_model::meal::Meal;
use crate::order_model::user::User;
use crate::util::money::Money;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Meals {
    /// Meal by unique ID
    meals: HashMap<u32, Meal>,
    owner: Rc<User>,
    /// Whether the meals selection has been completed
    ready: bool,
    paid: Money,
    tip: Money,
}

impl Meals {
    pub fn new(user: Rc<User>) -> Meals {
        Meals {
            meals: HashMap::new(),
            owner: user,
            ready: false,
            paid: Money::new(0, 0),
            tip: Money::new(0, 0),
        }
    }

    pub fn add_meal(&mut self, meal: Meal) -> &mut Meal {
        let id = meal.get_id();
        self.meals.insert(id, meal);
        self.meals.get_mut(&id).unwrap()
    }

    pub fn calculate_total_price(&self) -> Money {
        let mut total_price = Money::new(0,0);
        for (_, meal) in self.meals.iter() {
            total_price = total_price + meal.get_price();
        }
        return total_price;
    }

    // pub fn calculate_change(&self) -> Money {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::order_model::meal::MealFactory;

    #[test]
    fn meals_can_be_created() {
        // Given:
        let user = Rc::new(User::new(String::from("Peter")));

        // When:
        let meals = Meals::new(user.clone());

        // Then:
        assert_eq!(
            meals,
            Meals {
                meals: HashMap::new(),
                owner: user,
                ready: false,
                paid: Money::new(0, 0),
                tip: Money::new(0, 0),
            }
        );
    }

    #[test]
    fn meal_can_be_added_to_meals() {
        // Given:
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user.clone());

        let meal = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );

        // When:
        let added = meals.add_meal(meal);

        // Then:
        assert_eq!(
            added,
            &Meal::new(
                0,
                String::from("03"),
                String::from("groß"),
                Money::new(5, 50),
            )
        );

        let mut expected_meals = HashMap::new();
        expected_meals.insert(
            0,
            Meal::new(
                0,
                String::from("03"),
                String::from("groß"),
                Money::new(5, 50),
            ),
        );
        assert_eq!(
            meals,
            Meals {
                meals: expected_meals,
                owner: user,
                ready: false,
                paid: Money::new(0, 0),
                tip: Money::new(0, 0),
            }
        );
    }

    #[rstest(prices, expected_total,
        case(vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 33)], Money::new(15, 8)),
        case(vec![Money::new(3, 50), Money::new(4, 42)], Money::new(7, 92)),
    )]
    fn total_price_is_calculated_corectly(prices: Vec<Money>, expected_total: Money) {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user.clone());
        let mut meal_factory = MealFactory::new();

        for price in prices.into_iter() {
            let meal = meal_factory.create_meal(String::from("03"), String::from("groß"), price);
            meals.add_meal(meal);
        }
        //When
        let calculated_total = meals.calculate_total_price();
        //Then
        assert_eq!(expected_total, calculated_total)
    }
}
