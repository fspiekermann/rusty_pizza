use crate::order_model::meal::{Meal, MealFactory};
use crate::order_model::meals::Meals;
use crate::util::money::Money;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
/// Not all users who take part in this Order have paid enough.
pub enum NotAllPaidEnoughError {
    /// In total, not enough Money was paid
    Underpaid {
        underpaid: Money,
        /// IDs of the users that didn't pay enough
        paid_less: HashSet<u32>,
    },
    /// There is enough money to pay the bill, but somebody did not pay enough
    EnoughInTotal {
        change: Money,
        /// IDs of the users that didn't pay enough
        paid_less: HashSet<u32>,
    },
}

impl fmt::Display for NotAllPaidEnoughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NotAllPaidEnoughError::*;
        match &*self {
            Underpaid {
                underpaid,
                paid_less,
            } => write!(
                f,
                "There are missing {:?} total amount!\n{:?} underpaid",
                underpaid, paid_less
            ),
            EnoughInTotal { change, paid_less } => write!(
                f,
                "We have enough money and will get {:?} change!\nBut {:?} underpaid",
                change, paid_less
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
    /// Maps IDs of users to their `Meals`
    meals: HashMap<u32, Meals>,
    status: OrderStatus,
    /// User ID of the manager
    manager_id: u32,
    meal_factory: MealFactory,
}

impl Order {
    pub fn new(manager_id: u32) -> Order {
        Order {
            meals: HashMap::new(),
            status: OrderStatus::Open,
            manager_id,
            meal_factory: MealFactory::new(),
        }
    }

    pub fn add_user(&mut self, user_id: u32) -> &mut Meals {
        let meals = Meals::new(user_id);
        self.meals.insert(user_id, meals);
        self.meals.get_mut(&user_id).unwrap()
    }

    pub fn add_meal_for_user(
        &mut self,
        user_id: u32,
        meal_id: String,
        variety: String,
        price: Money,
    ) -> Result<&mut Meal, OrderError> {
        match self.meals.get_mut(&user_id) {
            Some(meals) => {
                let meal = self.meal_factory.create_meal(meal_id, variety, price);
                Ok(meals.add_meal(meal))
            }
            None => Err(OrderError::UserNotParticipating),
        }
    }

    pub fn get_meals_for_user(&mut self, user_id: u32) -> Option<&mut Meals> {
        self.meals.get_mut(&user_id)
    }

    pub fn calculate_total_price(&self) -> Money {
        let mut total_price = Money::zero();
        for single_order in self.meals.values() {
            total_price += single_order.calculate_total_price();
        }
        return total_price;
    }

    pub fn calculate_total_tip(&self) -> Money {
        let mut total_tip = Money::zero();
        for single_order in self.meals.values() {
            total_tip += single_order.get_tip();
        }
        return total_tip;
    }

    pub fn calculate_total_change(&self) -> Result<Money, NotAllPaidEnoughError> {
        let mut total_change = Money::zero();
        let mut underpaid = Money::zero();
        let mut paid_less: HashSet<u32> = HashSet::new();
        for single_order in self.meals.values() {
            match single_order.calculate_change() {
                Ok(change) => total_change += change,
                Err(e) => {
                    paid_less.insert(single_order.get_owner_id());
                    underpaid += e.get_value()
                }
            }
        }
        if underpaid.get_total_cents() == 0 {
            Ok(total_change)
        } else if total_change > underpaid {
            Err(NotAllPaidEnoughError::EnoughInTotal {
                change: total_change - underpaid,
                paid_less,
            })
        } else {
            Err(NotAllPaidEnoughError::Underpaid {
                underpaid: underpaid - total_change,
                paid_less,
            })
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
        let user_id = 0;
        //When
        let order = Order::new(user_id);
        //Then
        assert_eq!(order.meals.len(), 0);
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager_id, user_id);
    }

    #[test]
    fn user_can_be_added_to_order() {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        let user_id = 1;

        //When
        let meal = order.add_user(user_id);

        //Then
        assert_eq!(meal, &mut Meals::new(user_id));
        assert_eq!(order.meals.len(), 1);
        assert_eq!(order.meals[&user_id], Meals::new(user_id));
        assert_eq!(order.status, OrderStatus::Open);
        assert_eq!(order.manager_id, manager_id);
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
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        let user_id = 1;
        order.add_user(user_id);

        // When:
        let meal =
            order.add_meal_for_user(user_id, meal_id.clone(), variety.clone(), price.clone());

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
        let mut expected_meals = Meals::new(user_id);
        expected_meals.add_meal(Meal::new(0, meal_id, variety, price));
        assert_eq!(order.get_meals_for_user(user_id), Some(&mut expected_meals));
    }

    #[test]
    fn meal_cannot_be_added_to_order_for_user_if_user_is_not_participating_in_order() {
        // Given:
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        let user_id = 1;

        // When:
        let meal = order.add_meal_for_user(
            user_id,
            String::from("03"),
            String::from("groß"),
            Money::new(5, 50),
        );

        // Then:
        assert_eq!(meal, Err(OrderError::UserNotParticipating));
        assert_eq!(order.get_meals_for_user(user_id), None);
    }

    #[test]
    fn user_not_participating_in_order_has_no_meals() {
        // Given:
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        let user_id = 1;

        // When:
        let meals = order.get_meals_for_user(user_id);

        // Then:
        assert_eq!(meals, None);
    }

    #[test]
    fn user_participating_in_order_has_meals() {
        // Given:
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        let user_id = 1;
        order.add_user(user_id);

        // When:
        let meals = order.get_meals_for_user(user_id);

        // Then:
        assert_eq!(meals, Some(&mut Meals::new(user_id)));
    }

    struct MealsAttributes {
        meal_price: Vec<Money>,
        orderer_id: u32,
        amount: Money,
    }

    #[rstest(meals_attributes, expected_total,
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
                    orderer_id: 1,
                    amount: Money::zero(),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(3, 50), Money::new(4, 42)],
                    orderer_id: 2,
                    amount: Money::zero(),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(6, 83)],
                    orderer_id: 3,
                    amount: Money::zero(),
                },
            ],
            Money::new(29, 87)),
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(4, 42)],
                    orderer_id: 1,
                    amount: Money::zero(),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(5, 50)],
                    orderer_id: 2,
                    amount: Money::zero(),
                },
            ],
            Money::new(12, 17)),
    )]
    fn total_price_is_calculated_correctly(
        meals_attributes: Vec<MealsAttributes>,
        expected_total: Money,
    ) {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        for attributes in meals_attributes.into_iter() {
            order.add_user(attributes.orderer_id);
            for price in attributes.meal_price.iter() {
                order
                    .add_meal_for_user(
                        attributes.orderer_id,
                        String::from("XX"),
                        String::from("something"),
                        price.clone(),
                    )
                    .unwrap();
            }
        }
        //When
        let calculated_total = order.calculate_total_price();
        //Then
        assert_eq!(expected_total, calculated_total);
    }

    #[rstest(meals_attributes, total_tip,
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![],
                    orderer_id: 1,
                    amount: Money::new(2, 25),
                },
                MealsAttributes {
                    meal_price: vec![],
                    orderer_id: 2,
                    amount: Money::new(5, 50),
                },
                MealsAttributes {
                    meal_price: vec![],
                    orderer_id: 3,
                    amount: Money::new(7, 37),
                },
            ],
            Money::new(15, 12)),
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![],
                    orderer_id: 1,
                    amount: Money::new(2, 25),
                },
                MealsAttributes {
                    meal_price: vec![],
                    orderer_id: 2,
                    amount: Money::new(4, 42),
                },
            ],
            Money::new(6, 67)),
    )]
    fn total_tip_is_calculated_correctly(meals_attributes: Vec<MealsAttributes>, total_tip: Money) {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        for attributes in meals_attributes.into_iter() {
            order.add_user(attributes.orderer_id);
            order
                .get_meals_for_user(attributes.orderer_id)
                .unwrap()
                .set_tip(attributes.amount)
        }

        //When
        let calculated_tip = order.calculate_total_tip();

        //Then
        assert_eq!(total_tip, calculated_tip);
    }

    #[rstest(meals_attributes, expected_change,
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
                    orderer_id: 1,
                    amount: Money::new(17, 00),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(3, 50), Money::new(4, 42)],
                    orderer_id: 2,
                    amount: Money::new(8, 50),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(6, 83)],
                    orderer_id: 3,
                    amount: Money::new(6, 83),
                },
            ],
            Money::new(2, 46),
        ),
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(4, 42)],
                    orderer_id: 1,
                    amount: Money::new(8, 25),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(5, 50)],
                    orderer_id: 2,
                    amount: Money::new(5, 50),
                },
            ],
            Money::new(1, 58),
        ),
    )]
    fn all_paid_enough_change_is_calculated_correctly(
        meals_attributes: Vec<MealsAttributes>,
        expected_change: Money,
    ) {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        for attributes in meals_attributes.into_iter() {
            order.add_user(attributes.orderer_id);
            for price in attributes.meal_price.iter() {
                order
                    .add_meal_for_user(
                        attributes.orderer_id,
                        String::from("XX"),
                        String::from("something"),
                        price.clone(),
                    )
                    .unwrap();
            }
            order
                .get_meals_for_user(attributes.orderer_id)
                .unwrap()
                .set_paid(attributes.amount)
        }
        //When
        let calculated_change = order.calculate_total_change().unwrap();
        //Then
        assert_eq!(expected_change, calculated_change);
    }

    fn build_paid_less_hash_set(user_ids: Vec<u32>) -> HashSet<u32> {
        user_ids.into_iter().collect()
    }

    #[rstest(meals_attributes, expected_change,
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
                    orderer_id: 1,
                    amount: Money::new(17, 00),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(3, 50), Money::new(4, 42)],
                    orderer_id: 2,
                    amount: Money::new(7, 50),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(6, 83)],
                    orderer_id: 3,
                    amount: Money::new(6, 00),
                },
            ],
            NotAllPaidEnoughError::EnoughInTotal{
                change: Money::new(0, 63),
                paid_less: build_paid_less_hash_set(vec!(2, 3)),
            },
        ),
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(4, 42)],
                    orderer_id: 1,
                    amount: Money::new(8, 25),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(5, 50)],
                    orderer_id: 2,
                    amount: Money::new(5, 00),
                },
            ],
            NotAllPaidEnoughError::EnoughInTotal{
                change: Money::new(1, 08),
                paid_less: build_paid_less_hash_set(vec!(2)),
            },
        ),
    )]
    fn not_all_paid_enough_change_is_positive(
        meals_attributes: Vec<MealsAttributes>,
        expected_change: NotAllPaidEnoughError,
    ) {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        for attributes in meals_attributes.into_iter() {
            order.add_user(attributes.orderer_id);
            for price in attributes.meal_price.iter() {
                order
                    .add_meal_for_user(
                        attributes.orderer_id,
                        String::from("XX"),
                        String::from("something"),
                        price.clone(),
                    )
                    .unwrap();
            }
            order
                .get_meals_for_user(attributes.orderer_id)
                .unwrap()
                .set_paid(attributes.amount)
        }
        //When
        let calculated_change = order.calculate_total_change();
        //Then
        assert_eq!(Err(expected_change), calculated_change);
    }

    #[rstest(meals_attributes, expected_change,
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(5, 50), Money::new(7, 37)],
                    orderer_id: 1,
                    amount: Money::new(16, 00),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(3, 50), Money::new(4, 42)],
                    orderer_id: 2,
                    amount: Money::new(7, 50),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(6, 83)],
                    orderer_id: 3,
                    amount: Money::new(6, 00),
                },
            ],
            NotAllPaidEnoughError::Underpaid{
                underpaid: Money::new(0, 37),
                paid_less: build_paid_less_hash_set(vec!(2, 3)),
            },
        ),
        case(
            vec![
                MealsAttributes {
                    meal_price: vec![Money::new(2, 25), Money::new(4, 42)],
                    orderer_id: 1,
                    amount: Money::new(6, 25),
                },
                MealsAttributes {
                    meal_price: vec![Money::new(5, 50)],
                    orderer_id: 2,
                    amount: Money::new(5, 00),
                },
            ],
            NotAllPaidEnoughError::Underpaid{
                underpaid: Money::new(0, 92),
                paid_less: build_paid_less_hash_set(vec!(1, 2)),
            },
        ),
    )]
    fn not_all_paid_enough_change_is_negative(
        meals_attributes: Vec<MealsAttributes>,
        expected_change: NotAllPaidEnoughError,
    ) {
        //Given
        let manager_id = 0;
        let mut order = Order::new(manager_id);

        for attributes in meals_attributes.into_iter() {
            order.add_user(attributes.orderer_id);
            for price in attributes.meal_price.iter() {
                order
                    .add_meal_for_user(
                        attributes.orderer_id,
                        String::from("XX"),
                        String::from("something"),
                        price.clone(),
                    )
                    .unwrap();
            }
            order
                .get_meals_for_user(attributes.orderer_id)
                .unwrap()
                .set_paid(attributes.amount)
        }
        //When
        let calculated_change = order.calculate_total_change();
        //Then
        assert_eq!(Err(expected_change), calculated_change);
    }
}
