use std::rc::Rc;

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

#[derive(Debug, PartialEq)]
struct Meals {
    meals: Vec<Meal>,
    owner: Rc<User>,
    paid: f64,
    tip: f64,
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
    meals: Vec<Meals>,
    status: OrderStatus,
    manager: Rc<User>,
}

impl Order {
    fn new(manager: Rc<User>) -> Order {
        Order {
            meals: Vec::new(),
            status: OrderStatus::Open,
            manager,
        }
    }
}

#[derive(Debug, PartialEq)]
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
        let name = String::from("Peter");
        let user = Rc::new(User { name: name });
        //When
        let order = Order::new(user.clone());
        //Then
        assert_eq!(order.meals.len(), 0);
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, user);
    }
}
