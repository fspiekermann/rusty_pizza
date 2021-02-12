#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> User {
        User { name }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_name(&self) -> String {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_can_be_created() {
        //Given
        let name = String::from("Peter")
        //When
        let user = User::new(&name)
        //Then
        assert_eq!(name, user.get_name())
    }
}