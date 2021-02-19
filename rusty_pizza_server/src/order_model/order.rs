use crate::order_model::meal::{Meal, MealFactory};
use crate::order_model::meals::Meals;
use crate::order_model::user::User;
use crate::util::money::Money;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::rc::Rc;

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
pub enum OrderError {
    UserNotParticipating,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderError::UserNotParticipating => write!(f, "user is not participating in order"),
        }
    }
}

impl error::Error for OrderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            OrderError::UserNotParticipating => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Order {
    meals: HashMap<Rc<User>, Meals>,
    status: OrderStatus,
    manager: Rc<User>,
    meal_factory: MealFactory,
}

impl Order {
    pub fn new(manager: Rc<User>) -> Order {
        Order {
            meals: HashMap::new(),
            status: OrderStatus::Open,
            manager,
            meal_factory: MealFactory::new(),
        }
    }

    pub fn add_user(&mut self, user: Rc<User>) -> &mut Meals {
        let meals = Meals::new(user.clone());
        self.meals.insert(user.clone(), meals);
        self.meals.get_mut(&user).unwrap()
    }

    pub fn add_meal_for_user(
        &mut self,
        user: Rc<User>,
        meal_id: String,
        variety: String,
        price: Money,
    ) -> Result<&mut Meal, OrderError> {
        match self.meals.get_mut(&user) {
            Some(meals) => {
                let meal = self.meal_factory.create_meal(meal_id, variety, price);
                Ok(meals.add_meal(meal))
            }
            None => Err(OrderError::UserNotParticipating),
        }
    }

    pub fn get_meals_for_user(&mut self, user: Rc<User>) -> Option<&mut Meals> {
        self.meals.get_mut(&user)
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
        let meal = order.add_user(user.clone());

        //Then
        assert_eq!(meal, &mut Meals::new(user.clone()));
        assert_eq!(order.meals.len(), 1);
        assert_eq!(order.meals[&user], Meals::new(user.clone()));
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager, manager);
    }

    #[rstest(
        status,
        expected,
        case(OrderStatus::Open, String::from("Open")),
        case(OrderStatus::Ordering, String::from("Ordering")),
        case(
            OrderStatus::Ordered(String::from("12:15")),
            String::from("Ordered(\"12:15\")")
        ),
        case(OrderStatus::Delivered, String::from("Delivered"))
    )]
    fn order_status_is_formatted_correctly(status: OrderStatus, expected: String) {
        assert_eq!(expected, status.to_string())
    }

    #[rstest(
        meal_id,
        variety,
        price,
        case(String::from("03"), String::from("groß"), Money::new(5, 50)),
        case(String::from("04"), String::from("riesig"), Money::new(7, 20))
    )]
    fn meal_can_be_added_to_order_for_user(meal_id: String, variety: String, price: Money) {
        // Given:
        let manager = Rc::new(User::new(String::from("Peter")));
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User::new(String::from("Petra")));
        order.add_user(user.clone());

        // When:
        let meal = order.add_meal_for_user(
            user.clone(),
            meal_id.clone(),
            variety.clone(),
            price.clone(),
        );

        // Then:
        assert_eq!(
            meal,
            Ok(&mut Meal::new(
                0,
                meal_id.clone(),
                variety.clone(),
                price.clone()
            ))
        );
        let mut expected_meals = Meals::new(user.clone());
        expected_meals.add_meal(Meal::new(0, meal_id, variety, price));
        assert_eq!(order.get_meals_for_user(user), Some(&mut expected_meals));
    }

    #[test]
    fn meal_cannot_be_added_to_order_for_user_if_user_is_not_participating_in_order() {
        // Given:
        let manager = Rc::new(User::new(String::from("Peter")));
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User::new(String::from("Petra")));

        // When:
        let meal = order.add_meal_for_user(
            user.clone(),
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );

        // Then:
        assert_eq!(meal, Err(OrderError::UserNotParticipating));
        assert_eq!(order.get_meals_for_user(user), None);
    }

    #[test]
    fn user_not_participating_in_order_has_no_meals() {
        // Given:
        let manager = Rc::new(User::new(String::from("Peter")));
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User::new(String::from("Petra")));

        // When:
        let meals = order.get_meals_for_user(user);

        // Then:
        assert_eq!(meals, None);
    }

    #[test]
    fn user_participating_in_order_has_meals() {
        // Given:
        let manager = Rc::new(User::new(String::from("Peter")));
        let mut order = Order::new(manager.clone());

        let user = Rc::new(User::new(String::from("Petra")));
        order.add_user(user.clone());

        // When:
        let meals = order.get_meals_for_user(user.clone());

        // Then:
        assert_eq!(meals, Some(&mut Meals::new(user)));
    }
}
