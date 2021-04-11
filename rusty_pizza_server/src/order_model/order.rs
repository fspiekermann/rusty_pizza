use crate::order_model::meal::{Meal, MealFactory};
use crate::order_model::meals::Meals;
use crate::order_model::user::User;
use crate::util::money::Money;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum NotAllPaidEnoughError {
    Underpaid(Money, HashSet<Rc<User>>), // Contains the difference between paid money and has to pay
    EnoughMoney(Money, HashSet<Rc<User>>),
}

impl fmt::Display for NotAllPaidEnoughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NotAllPaidEnoughError::*;
        match &*self {
            Underpaid(missing, paid_less) => write!(
                f,
                "There are missing {},{}Euro total amount!\n{:?} underpaid",
                missing.get_euros(),
                missing.get_cents(),
                paid_less
            ),
            EnoughMoney(missing, paid_less) => write!(
                f,
                "We have enough money and will get {},{}Euro change!\nBut {:?} underpaid",
                missing.get_euros(),
                missing.get_cents(),
                paid_less
            ),
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

    pub fn set_paid_for_user(
        &mut self,
        user: Rc<User>,
        paid: Money,
    ) -> Result<(), OrderError> {
        match self.meals.get_mut(&user) {
            Some(meals) => {
                meals.set_paid(paid);
                Ok(())
            }
            None => Err(OrderError::UserNotParticipating),
        }
    }

    pub fn set_tip_for_user(
        &mut self,
        user: Rc<User>,
        tip: Money,
    ) -> Result<(), OrderError> {
        match self.meals.get_mut(&user) {
            Some(meals) => {
                meals.set_tip(tip);
                Ok(())
            }
            None => Err(OrderError::UserNotParticipating),
        }
    }

    pub fn get_meals_for_user(&mut self, user: Rc<User>) -> Option<&mut Meals> {
        self.meals.get_mut(&user)
    }

    pub fn calculate_total_price(&self) -> Money {
        let mut total_price = Money::new(0, 0);
        for single_order in self.meals.values() {
            total_price += single_order.calculate_total_price();
        }
        return total_price;
    }

    pub fn calculate_total_tip(&self) -> Money {
        let mut total_tip = Money::new(0, 0);
        for single_order in self.meals.values() {
            total_tip += single_order.get_tip();
        }
        return total_tip;
    }

    pub fn calculate_total_change(&self) -> Result<Money, NotAllPaidEnoughError> {
        let mut total_change = Money::new(0, 0);
        let mut underpaid = Money::new(0, 0);
        let mut paid_less: HashSet<Rc<User>> = HashSet::new();
        for single_order in self.meals.values() {
            match single_order.calculate_change() {
                Ok(change)  => total_change += change,
                Err(e) => {
                    paid_less.insert(single_order.get_owner());
                    underpaid += e.get_value()
                }
            }
        }
        if underpaid.get_total_cents() == 0 {
            Ok(total_change)
        } else if total_change > underpaid {
            Err(NotAllPaidEnoughError::EnoughMoney(total_change - underpaid, paid_less))
        } else {
            Err(NotAllPaidEnoughError::Underpaid(underpaid - total_change, paid_less))
        }
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

    #[rstest(prices, names, expected_total,
        case(
            vec![
                vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
                vec![Money::new(3, 50), Money::new(4, 42)],
                vec![Money::new(6, 83)],
            ],
            vec![String::from("Peter"), String::from("Mia"), String::from("Harald")],
            Money::new(29, 87)),
        case(
            vec![
                vec![Money::new(2, 25), Money::new(4, 42)],
                vec![Money::new(5, 50)],
            ],
            vec![String::from("Adam"), String::from("Eva")],
            Money::new(12, 17)),
    )]
    fn total_price_is_calculated_correctly(prices: Vec<Vec<Money>>, names: Vec<String>, expected_total: Money) {
        //Given
        let manager = Rc::new(User::new(String::from("Gott")));
        let mut order = Order::new(manager);

        let mut names_iter = names.into_iter();
        for meal_prices in prices.into_iter() {
            let user = Rc::new(User::new(names_iter.next().unwrap()));
            order.add_user(user.clone());
            for price in meal_prices.iter() {
                order.add_meal_for_user(user.clone(), String::from("XX"), String::from("something"), price.clone()).unwrap();
            }
        }
        //When
        let calculated_total = order.calculate_total_price();
        //Then
        assert_eq!(expected_total, calculated_total);
    }

    #[rstest(tips, names, total_tip,
        case(
            vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
            vec![String::from("Peter"), String::from("Mia"), String::from("Harald")],
            Money::new(15, 12)),
        case(
            vec![Money::new(2, 25), Money::new(4, 42)],
            vec![String::from("Adam"), String::from("Eva")],
            Money::new(6, 67)),
    )]
    fn total_tip_is_calculated_correctly(tips: Vec<Money>, names: Vec<String>, total_tip: Money) {
        //Given
        let manager = Rc::new(User::new(String::from("Gott")));
        let mut order = Order::new(manager);

        let mut names_iter = names.into_iter();
        for tip in tips.into_iter() {
            let user = Rc::new(User::new(names_iter.next().unwrap()));
            order.add_user(user.clone());
            order.set_tip_for_user(user.clone(), tip).unwrap();
        }
        //When
        let calculated_tip = order.calculate_total_tip();

        //Then
        assert_eq!(total_tip, calculated_tip);
    }

    #[rstest(prices, paids, names, expected_change,
        case(
            vec![
                vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)], //15,13
                vec![Money::new(3, 50), Money::new(4, 42)], //7,92
                vec![Money::new(6, 83)], //6,83
            ],
            vec![Money::new(17, 00), Money::new(8, 50), Money::new(6, 83)],
            vec![String::from("Peter"), String::from("Mia"), String::from("Harald")],
            Money::new(2, 46),
        ),
        case(
            vec![
                vec![Money::new(2, 25), Money::new(4, 42)], //6,67
                vec![Money::new(5, 50)], //5,50
            ],
            vec![Money::new(8, 25), Money::new(5, 50)],
            vec![String::from("Adam"), String::from("Eva")],
            Money::new(1, 58),
        ),
    )]
    fn all_paid_enough_change_is_calculated_correctly(
        prices: Vec<Vec<Money>>,
        paids: Vec<Money>,
        names: Vec<String>,
        expected_change: Money
    ) {
        //Given
        let manager = Rc::new(User::new(String::from("Gott")));
        let mut order = Order::new(manager);

        let mut names_iter = names.into_iter();
        let mut paids_iter = paids.into_iter();
        for meal_prices in prices.into_iter() {
            let user = Rc::new(User::new(names_iter.next().unwrap()));
            let paid = paids_iter.next().unwrap();
            order.add_user(user.clone());
            for price in meal_prices.iter() {
                order.add_meal_for_user(user.clone(), String::from("XX"), String::from("something"), price.clone()).unwrap();
            }
            order.set_paid_for_user(user.clone(), paid).unwrap();
        }
        //When
        let calculated_change = order.calculate_total_change().unwrap();
        //Then
        assert_eq!(expected_change, calculated_change);
    }

    fn build_paid_less_hash_set(names: Vec<String>) -> HashSet<Rc<User>> {
        let mut paid_less: HashSet<Rc<User>> = HashSet::new();
        for name in names {
            paid_less.insert(Rc::new(User::new(name)));
        }
        return paid_less;
    }

    #[rstest(prices, paids, names, expected_change,
        case(
            vec![
                vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)], //15,13
                vec![Money::new(3, 50), Money::new(4, 42)], //7,92
                vec![Money::new(6, 83)], //6,83
            ],
            vec![Money::new(17, 00), Money::new(7, 50), Money::new(6, 00)],
            vec![String::from("Peter"), String::from("Mia"), String::from("Harald")],
            NotAllPaidEnoughError::EnoughMoney(
                Money::new(0, 63),
                build_paid_less_hash_set(vec!(String::from("Mia"), String::from("Harald"))),
            ),
        ),
        case(
            vec![
                vec![Money::new(2, 25), Money::new(4, 42)], //6,67
                vec![Money::new(5, 50)], //5,50
            ],
            vec![Money::new(8, 25), Money::new(5, 00)],
            vec![String::from("Adam"), String::from("Eva")],
            NotAllPaidEnoughError::EnoughMoney(
                Money::new(1, 08),
                build_paid_less_hash_set(vec!(String::from("Eva"))),
            ),
        ),
    )]
    fn not_all_paid_enough_change_is_positive(
        prices: Vec<Vec<Money>>,
        paids: Vec<Money>,
        names: Vec<String>,
        expected_change: NotAllPaidEnoughError
    ) {
        //Given
        let manager = Rc::new(User::new(String::from("Gott")));
        let mut order = Order::new(manager);

        let mut names_iter = names.into_iter();
        let mut paids_iter = paids.into_iter();
        for meal_prices in prices.into_iter() {
            let user = Rc::new(User::new(names_iter.next().unwrap()));
            let paid = paids_iter.next().unwrap();
            order.add_user(user.clone());
            for price in meal_prices.iter() {
                order.add_meal_for_user(user.clone(), String::from("XX"), String::from("something"), price.clone()).unwrap();
            }
            order.set_paid_for_user(user.clone(), paid).unwrap();
        }
        //When
        let calculated_change = order.calculate_total_change();
        //Then
        assert_eq!(Err(expected_change), calculated_change);
    }

    #[rstest(prices, paids, names, expected_change,
        case(
            vec![
                vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)], //15,13
                vec![Money::new(3, 50), Money::new(4, 42)], //7,92
                vec![Money::new(6, 83)], //6,83
            ],
            vec![Money::new(16, 00), Money::new(7, 50), Money::new(6, 00)],
            vec![String::from("Peter"), String::from("Mia"), String::from("Harald")],
            NotAllPaidEnoughError::Underpaid(
                Money::new(0, 37),
                build_paid_less_hash_set(vec!(String::from("Mia"), String::from("Harald"))),
            ),
        ),
        case(
            vec![
                vec![Money::new(2, 25), Money::new(4, 42)], //6,67
                vec![Money::new(5, 50)], //5,50
            ],
            vec![Money::new(6, 25), Money::new(5, 00)],
            vec![String::from("Adam"), String::from("Eva")],
            NotAllPaidEnoughError::Underpaid(
                Money::new(0, 92),
                build_paid_less_hash_set(vec!(String::from("Adam"), String::from("Eva"))),
            ),
        ),
    )]
    fn not_all_paid_enough_change_is_negative(
        prices: Vec<Vec<Money>>,
        paids: Vec<Money>,
        names: Vec<String>,
        expected_change: NotAllPaidEnoughError
    ) {
        //Given
        let manager = Rc::new(User::new(String::from("Gott")));
        let mut order = Order::new(manager);

        let mut names_iter = names.into_iter();
        let mut paids_iter = paids.into_iter();
        for meal_prices in prices.into_iter() {
            let user = Rc::new(User::new(names_iter.next().unwrap()));
            let paid = paids_iter.next().unwrap();
            order.add_user(user.clone());
            for price in meal_prices.iter() {
                order.add_meal_for_user(user.clone(), String::from("XX"), String::from("something"), price.clone()).unwrap();
            }
            order.set_paid_for_user(user.clone(), paid).unwrap();
        }
        //When
        let calculated_change = order.calculate_total_change();
        //Then
        assert_eq!(Err(expected_change), calculated_change);
    }
}
