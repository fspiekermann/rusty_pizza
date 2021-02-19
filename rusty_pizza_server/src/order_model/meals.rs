use crate::order_model::meal::Meal;
use crate::order_model::user::User;
use crate::util::money::Money;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Meals {
    /// Meal and quantity
    meals: HashMap<Meal, i32>,
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

    pub fn add_meal(&mut self, meal: Meal) {
        self.meals.insert(meal, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meals_can_be_created() {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        //When
        let meals = Meals::new(user.clone());
        //Then
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
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        let mut meals = Meals::new(user.clone());

        let meal = Meal::new(
            0,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );

        //When
        meals.add_meal(meal);

        //Then
        let mut expected_meals = HashMap::new();
        expected_meals.insert(
            Meal::new(
                0,
                String::from("03"),
                String::from("groß"),
                Money::new(5, 50),
            ),
            1,
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
}
