use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;
use std::rc::Rc;

mod money;
use money::Money;

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

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Open,
    Ordering,
    Ordered(String),
    Delivered,
}

#[derive(Debug, PartialEq)]
struct Order {
    meals: HashMap<Rc<User>, Meals>,
    status: OrderStatus,
    manager: Rc<User>,
}

impl Order {
    fn new(manager: Rc<User>) -> Order {
        Order {
            meals: HashMap::new(),
            status: OrderStatus::Open,
            manager,
        }
    }

    fn add_user(&mut self, user: Rc<User>) {
        let meals = Meals::new(user.clone());
        self.meals.insert(user, meals);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct User {
    name: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_can_be_created() {
        //Given
        let user = Rc::new(User {
            name: String::from("Peter"),
        });
        //When
        let order = Order::new(user.clone());
        //Then
        assert_eq!(order.meals.len(), 0);
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, user);
    }

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
    fn user_can_be_added_to_order() {
        //Given
        let manager = Rc::new(User {
            name: String::from("Peter"),
        });
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User {
            name: String::from("Karl"),
        });

        //When
        order.add_user(user.clone());

        //Then
        assert_eq!(order.meals.len(), 1);
        assert_eq!(
            order.meals[&user],
            Meals {
                meals: HashMap::new(),
                owner: user,
                ready: false,
                paid: Money::new(0, 0),
                tip: Money::new(0, 0),
            }
        );
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, manager);
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
