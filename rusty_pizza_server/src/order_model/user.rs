use crate::util::id::Id;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User {
    id: Id,
    name: String,
}

impl User {
    pub fn new(id: Id, name: String) -> User {
        User { id, name }
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_can_be_created() {
        //Given
        let id = Id::new(42);
        let str_name = "Peter";
        let name = String::from(str_name);

        //When
        let user = User::new(id.clone(), name);

        //Then
        assert_eq!(id, user.get_id());
        assert_eq!(str_name, user.get_name());
    }
}
