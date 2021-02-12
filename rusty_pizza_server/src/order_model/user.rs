#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> User {
        User { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_can_be_created() {
        //Given
        let str_name = "Peter";
        let name = String::from(str_name);
        //When
        let user = User::new(name);
        //Then
        assert_eq!(str_name, user.get_name());
    }
}