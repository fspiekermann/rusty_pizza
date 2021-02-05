use std::rc::Rc;
use std::collections::HashMap;

// TODO: calculate change money
#[derive(Debug, PartialEq)]
struct Meal {
    /// Number of the meal in the menu
    meal_id: String,
    /// Size of the pizza or noodle type etc.
    variety: String,
    price: f64,
    specials: Vec<String>,
}

impl Meal {
    fn add_special(&mut self, special: String) {
        self.specials.push(special);
    }
}

#[derive(Debug, PartialEq)]
struct Meals {
    meals: Vec<Meal>,
    owner: Rc<User>,
    /// Whether the meals selection has been completed
    ready: bool,
    paid: f64,
    tip: f64,
}

impl Meals {
    fn new(user: Rc<User>) -> Meals {
        Meals {
            meals: Vec::new(),
            owner: user,
            ready: false,
            paid: 0.0,
            tip: 0.0,
        }
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
        let user = Rc::new(User { name: String::from("Peter") });
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
        let user = Rc::new(User { name: String::from("Peter") });
        //When
        let meals = Meals::new(user.clone());
        //Then
        assert_eq!(meals, Meals {
            meals: vec![],
            owner: user,
            ready: false,
            paid: 0.0,
            tip: 0.0,
        });
    }

    #[test]
    fn user_can_be_added_to_order() {
        //Given
        let manager = Rc::new(User { name: String::from("Peter") });
        let mut order = Order::new(manager.clone());
        
        let user = Rc::new(User { name: String::from("Karl") });

        //When
        order.add_user(user.clone());

        //Then
        assert_eq!(order.meals.len(), 1);
        assert_eq!(order.meals[&user], Meals {
            meals: vec![],
            owner: user,
            ready: false,
            paid: 0.0,
            tip: 0.0,
        });
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, manager);
    }

    #[test]
    fn special_can_be_added_to_meal() {
        //Given
        let mut meal = Meal {
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: 5.50,
            specials: Vec::new(),
        };

        let special = String::from("Käserand");

        //When
        meal.add_special(special);

        //Then
        assert_eq!(meal, Meal {
            meal_id: String::from("03"),
            variety: String::from("groß"),
            price: 5.50,
            specials: vec![String::from("Käserand")],
        });
    }
}
