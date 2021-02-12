use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;
use crate::order_model::user::User;
use crate::order_model::meals::Meals;
use crate::util::money::Money;

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Open,
    Ordering,
    Ordered(String),
    Delivered,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Order {
    meals: HashMap<Rc<User>, Meals>,
    status: OrderStatus,
    manager: Rc<User>,
}

impl Order {
    pub fn new(manager: Rc<User>) -> Order {
        Order {
            meals: HashMap::new(),
            status: OrderStatus::Open,
            manager,
        }
    }

    pub fn add_user(&mut self, user: Rc<User>) {
        let meals = Meals::new(user.clone());
        self.meals.insert(user, meals);
    }

    pub fn get_order_status(self) -> String {
        self.status.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn order_can_be_created() {
        //Given
        let user = Rc::new(User::new(String::from("Peter")));
        //When
        let order = Order::new(user.clone());
        //Then
        assert_eq!(order.meals.len(), 0);
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, user);
    }

    #[test]
    fn user_can_be_added_to_order() {
        //Given
        let manager = Rc::new(User::new(String::from("Peter")));
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User::new(String::from("Petra")));

        //When
        order.add_user(user.clone());

        //Then
        assert_eq!(order.meals.len(), 1);
        assert_eq!(
            order.meals[&user],
            Meals::new_for_test(user.clone(), false)
        );
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, manager);
    }

    #[rstest(status, expected,
        case(OrderStatus::Open, String::from("Open")),
        case(OrderStatus::Ordering, String::from("Ordering")),
        case(OrderStatus::Ordered(String::from("12:15")), String::from("Ordered(\"12:15\")")),
        case(OrderStatus::Delivered, String::from("Delivered")),
    )]
    fn order_status_is_formatted_correctly(status: OrderStatus, expected: String) {
        assert_eq!(expected, status.to_string())
    }
}