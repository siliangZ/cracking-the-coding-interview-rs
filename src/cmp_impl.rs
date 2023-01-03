//! implement Ord and PartialOrd on self-defined type

struct Student {
    id: u32,
    name: String,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

// implement PartialOrd needs to implement PartialEq first
// >, <, >=, <=, ==, !=
impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.name.partial_cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Student;

    #[test]
    fn test_order() {
        let s1 = Student {
            id: 0,
            name: String::from("a"),
        };
        let s2 = Student {
            id: 1,
            name: String::from("b"),
        };
        let s3 = Student {
            id: 0,
            name: String::from("b"),
        };

        assert!(s1 < s2);
        assert!(s1 < s3);
    }
}
