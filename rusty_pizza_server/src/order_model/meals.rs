use std::collections::HashMap;
use std::rc::Rc;
use crate::util::money::Money;

#[derive(Debug, PartialEq)]
struct Meals {
    /// Meal and quantity
    meals: HashMap<Meal, i32>,
    owner: Rc<User>,
    /// Whether the meals selection has been completed
    ready: bool,
    paid: Money,
    tip: Money,
}

impl Meals {
    fn new(user: Rc<User>) -> Meals {
        Meals {
            meals: HashMap::new(),
            owner: user,
            ready: false,
            paid: Money::new(0, 0),
            tip: Money::new(0, 0),
        }
    }

    fn add_meal(&mut self, meal: Meal) {
        self.meals.insert(meal, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meals_can_be_created() {
        //Given
        let user = Rc::new(User {
            name: String::from("Peter"),
        });
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
        let user = Rc::new(User {
            name: String::from("Peter"),
        });
        let mut meals = Meals::new(user.clone());

        let meal = Meal {
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: Money::new(5, 50),
            specials: BTreeSet::new(),
        };

        //When
        meals.add_meal(meal);

        //Then
        let mut expected_meals = HashMap::new();
        expected_meals.insert(
            Meal {
                meal_id: String::from("03"),
                variety: String::from("groß"),
                price: Money::new(5, 50),
                specials: BTreeSet::new(),
            },
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