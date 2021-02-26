use crate::order_model::meal::Meal;
use crate::order_model::user::User;
use crate::util::money::Money;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum ChangeMoneyError {
    Underpaid(Money), // Contains the difference between paid money and has to pay 
}

impl fmt::Display for ChangeMoneyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ChangeMoneyError::*;
        match *self {
            Underpaid(missing) => write!(
                f,
                "You have underpaid by {},{}Euro ",
                missing.get_euros(),
                missing.get_cents()
            ),
        }
    }
}

impl Error for ChangeMoneyError {}

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

    pub fn set_paid(&mut self, paid: Money) {
        self.paid = paid;
    }

    pub fn set_tip(&mut self, tip: Money) {
        self.tip = tip;
    }

    pub fn calculate_total_price(&self) -> Money {
        let mut total_price = Money::new(0, 0);
        for meal in self.meals.values() {
            total_price = total_price + meal.get_price();
        }
        return total_price;
    }

    pub fn calculate_change(&self) -> Result<Money, ChangeMoneyError> {
        let has_to_pay = self.calculate_total_price() + self.tip;
        if self.paid.get_total_cents() < has_to_pay.get_total_cents() {
            return Err(ChangeMoneyError::Underpaid(has_to_pay - self.paid));
        }
        return Ok(self.paid - has_to_pay);
    pub fn remove_meal(&mut self, meal: Meal) -> bool {
        // let id = meal.get_id();
        // self.meals.insert(id, meal);
        // self.meals.get_mut(&id).unwrap()
        return false;
    }

    pub fn remove_meal_by_id(&mut self, id: u32) -> Option<Meal> {
        // let id = meal.get_id();
        // self.meals.insert(id, meal);
        // self.meals.get_mut(&id).unwrap()
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order_model::meal::MealFactory;
    use rstest::rstest;

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
        case(vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 33)], Money::new(15, 08)),
        case(vec![Money::new(3, 50), Money::new(4, 42)], Money::new(7, 92)),
    )]
    fn total_price_is_calculated_correctly(prices: Vec<Money>, expected_total: Money) {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user);
        let mut meal_factory = MealFactory::new();

        for price in prices.into_iter() {
            let meal =
                meal_factory.create_meal(String::from("XX"), String::from("something"), price);
            meals.add_meal(meal);
        }
        //When
        let calculated_total = meals.calculate_total_price();
        //Then
        assert_eq!(expected_total, calculated_total);
    }

    #[rstest(prices, paid, tip, expected_change,
        case(vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 33)], Money::new(20, 0), Money::new(2, 20), Money::new(2, 72)),
        case(vec![Money::new(3, 50), Money::new(4, 42)], Money::new(10, 50), Money::new(1, 50), Money::new(1, 8)),
    )]
    fn positve_change_is_calculated_correctly(
        prices: Vec<Money>,
        paid: Money,
        tip: Money,
        expected_change: Money,
    ) {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user);
        meals.set_paid(paid);
        meals.set_tip(tip);
        let mut meal_factory = MealFactory::new();

        for price in prices.into_iter() {
            let meal =
                meal_factory.create_meal(String::from("XX"), String::from("something"), price);
            meals.add_meal(meal);
        }
        //When
        let calculated_change = meals.calculate_change().unwrap();
        //Then
        assert_eq!(expected_change, calculated_change);
    }

    #[rstest(prices, paid, tip, expected_change,
        case(vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 33)], Money::new(15, 0), Money::new(2, 20), ChangeMoneyError::Underpaid(Money::new(2, 28))),
        case(vec![Money::new(3, 50), Money::new(4, 42)], Money::new(7, 50), Money::new(1, 50), ChangeMoneyError::Underpaid(Money::new(1, 92))),
    )]
    fn negative_change_returns_underpaid(
        prices: Vec<Money>,
        paid: Money,
        tip: Money,
        expected_change: ChangeMoneyError,
    ) {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user);
        meals.set_paid(paid);
        meals.set_tip(tip);
        let mut meal_factory = MealFactory::new();

        for price in prices.into_iter() {
            let meal =
                meal_factory.create_meal(String::from("XX"), String::from("something"), price);
            meals.add_meal(meal);
        }
        //When
        let change = meals.calculate_change();
        //Then
        assert_eq!(Err(expected_change), change);

    #[rstest(to_remove, expected_result, remaining_length
        case(Meal::new(0, String::from("03"), String::from("groß"), Money::new(5, 50)), true, 1),
        case(Meal::new(1, String::from("35"), String::from("Spaghetti"), Money::new(4, 35)), true, 1),
        case(Meal::new(2, String::from("42"), String::from("Kräuterbutter"), Money::new(2, 25)), false, 2),
    )]
    fn meal_can_be_removed_from_meals(to_remove: Meal, expected_result: bool, remaining_length: u32) {
        // Given:
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user.clone());

        let meal_1 = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );
        let meal_2 = Meal::new(
            1,
            String::from("35"),
            String::from("Spaghetti"),
            Money::new(4, 35),
        );

        let added = meals.add_meal(meal_1);
        let added = meals.add_meal(meal_2);
        // When:
        let removed = meals.remove_meal(to_remove)
        // Then:
        assert_eq!(expected_result, removed)
        assert_eq!(remaining_length, meals.meal.len())
    }

    #[rstest(id, expected_removed, remaining_length
        case(0, Some(Meal::new(0, String::from("03"), String::from("groß"), Money::new(5, 50))), 1),
        case(1, Some(Meal::new(1, String::from("35"), String::from("Spaghetti"), Money::new(4, 35))), 1),
        case(2, None, 2),
    )]
    fn meal_can_be_removed_from_meals_by_id(id: u32, expected_removed: Option<Meal>, , remaining_length: u32) {
        // Given:
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user.clone());

        let meal_1 = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );
        let meal_2 = Meal::new(
            1,
            String::from("35"),
            String::from("Spaghetti"),
            Money::new(4, 35),
        );

        let added = meals.add_meal(meal_1);
        let added = meals.add_meal(meal_2);
        // When:
        let removed_meal = meals.remove_meal_by_id(id)
        // Then:
        assert_eq!(expected_removed, removed_meal)
        assert_eq!(remaining_length, meals.meal.len())
    }
}
