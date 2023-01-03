use std::collections::BinaryHeap;

use chrono::{DateTime, Utc};

struct Stack {
    arr: Vec<u8>,
    min: Vec<u8>,
}

impl Stack {
    pub fn push(&mut self, elem: u8) {
        self.arr.push(elem);
        if self.min.is_empty() || *self.min.last().unwrap() >= elem {
            self.min.push(elem);
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        let result = self.arr.pop();
        if result.is_some() && result.as_ref().eq(&self.min.last()) {
            self.min.pop();
        }
        result
    }

    pub fn min(&self) -> Option<&u8> {
        self.min.last()
    }
}

struct SetOfStacks {
    stacks: Vec<Vec<u8>>,
    threshold: usize,
}

impl SetOfStacks {
    pub fn new(threshold: usize) -> Self {
        Self {
            stacks: Vec::new(),
            threshold,
        }
    }

    pub fn push(&mut self, elem: u8) {
        if let Some(stack) = self.stacks.last_mut() {
            if stack.len() < self.threshold {
                stack.push(elem);
            } else {
                self.stacks.push(vec![elem]);
            }
        } else {
            self.stacks.push(vec![elem])
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if let Some(mut stack) = self.stacks.pop() {
            let elem = stack.pop();
            if !stack.is_empty() {
                self.stacks.push(stack);
            }
            elem
        } else {
            None
        }
    }
}

struct MyQueue {
    stack: Vec<u8>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, elem: u8) {
        self.stack.push(elem)
    }

    pub fn pop(&mut self) -> Option<u8> {
        let mut temp = Vec::new();
        while let Some(elem) = self.stack.pop() {
            temp.push(elem);
        }

        let result = temp.pop();
        while let Some(elem) = temp.pop() {
            self.stack.push(elem)
        }
        result
    }
}

fn sort_stack(mut stack: Vec<u8>) -> Vec<u8> {
    if stack.is_empty() {
        return vec![];
    }
    let mut temp = Vec::new();
    while let Some(elem) = stack.pop() {
        while let Some(last) = temp.last() {
            if *last >= elem {
                break;
            }
            if let Some(next) = temp.pop() {
                stack.push(next);
            }
        }
        temp.push(elem)
    }
    temp
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum AnimalType {
    Dog,
    Cat,
}

#[derive(PartialEq, Eq)]
struct Animal {
    name: String,
    animal_type: AnimalType,
    arrival_time: DateTime<Utc>,
}

impl Ord for Animal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.arrival_time.cmp(&self.arrival_time)
    }
}

impl PartialOrd for Animal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // does this one matter?
        Some(self.cmp(other))
    }
}

struct AnimalShelter {
    dogs: BinaryHeap<Animal>,
    cats: BinaryHeap<Animal>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        Self {
            dogs: BinaryHeap::new(),
            cats: BinaryHeap::new(),
        }
    }

    pub fn enqueue(&mut self, animal: Animal) {
        if animal.animal_type == AnimalType::Dog {
            self.dogs.push(animal)
        } else {
            self.cats.push(animal)
        }
    }

    pub fn dequeue_any(&mut self) -> Option<Animal> {
        // check the oldest animal
        if self.cats.is_empty() {
            return self.dogs.pop();
        }

        if self.dogs.is_empty() {
            return self.cats.pop();
        }

        if self.cats.peek() < self.dogs.peek() {
            self.cats.pop()
        } else {
            self.dogs.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{sort_stack, AnimalShelter};

    #[test]
    fn test_animal_shelter() {
        let mut animal_shelter = AnimalShelter::new();
        animal_shelter.enqueue(super::Animal {
            name: String::from("D1"),
            animal_type: super::AnimalType::Dog,
            arrival_time: Utc::now(),
        });
        animal_shelter.enqueue(super::Animal {
            name: String::from("D2"),
            animal_type: super::AnimalType::Dog,
            arrival_time: Utc::now(),
        });
        animal_shelter.enqueue(super::Animal {
            name: String::from("D3"),
            animal_type: super::AnimalType::Dog,
            arrival_time: Utc::now(),
        });

        assert_eq!(
            String::from("D1"),
            animal_shelter.dequeue_any().unwrap().name
        );
        assert_eq!(
            String::from("D2"),
            animal_shelter.dequeue_any().unwrap().name
        );
        assert_eq!(
            String::from("D3"),
            animal_shelter.dequeue_any().unwrap().name
        );
    }

    #[test]
    fn test_sort_stack() {
        let sorted = sort_stack(vec![34, 3, 31, 98, 92, 23]);
        assert_eq!(vec![98, 92, 34, 31, 23, 3], sorted);
    }

    #[test]
    fn ref_cmp() {
        let s1 = String::from("hello_world");
        let s1_ref = Some(&s1);
        //println!("s1 address: {:p}", s1_ref.unwrap());
        let s2 = String::from("hello_world");
        let s2_ref = Some(&s2);
        //println!("s2 address: {:p}", s2_ref.unwrap());
        assert!(s2_ref.eq(&s1_ref));
    }
}
