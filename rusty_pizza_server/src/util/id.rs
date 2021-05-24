/// A usually unique ID referencing an entity.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Id {
    value: u32,
}

impl Id {
    pub fn new(value: u32) -> Id {
        Id { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_with_equal_value_are_equal() {
        // When:
        let id1 = Id::new(0);
        let id2 = Id::new(0);

        // Then:
        assert_eq!(id1, id2);
    }
}
