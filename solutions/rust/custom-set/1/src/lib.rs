use std::{array, hash::{DefaultHasher, Hash, Hasher}};

const NUM_BUCKETS: usize = 10;

#[derive(Debug, Eq)]
pub struct CustomSet<T: Hash + Copy> {
    num_items: usize,
    items: [Vec<T>; NUM_BUCKETS],
}

impl<T: Hash + Copy + PartialEq + Eq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.num_items == other.num_items && 
        self.is_subset(other)
    }
}

impl<T: Hash + Copy + PartialEq + Eq> CustomSet<T> {
    fn get_index_with_hash(item: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish() as usize % NUM_BUCKETS
    }

    pub fn new(input: &[T]) -> Self {
        let items: [Vec<T>; NUM_BUCKETS] =  array::from_fn(|_| Vec::new());
        let mut this = Self {
            items,
            num_items: 0
        };
        for item in input {
            this.add(item.clone());
        }
        this
    }

    pub fn contains(&self, element: &T) -> bool {
        let index = Self::get_index_with_hash(&element);
        self.items[index].contains(element)
    }

    pub fn add(&mut self, element: T) {
        let index = Self::get_index_with_hash(&element);
        if !self.items[index].contains(&element) {
            self.items[index].push(element);
            self.num_items += 1;
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter()
            .all(|vec: &Vec<T>| vec.iter()
                .all(|single_elem| other.contains(&single_elem)
            )
        )
    }

    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.items.iter()
            .all(|vec: &Vec<T>| vec.iter()
                .all(|single_elem| !other.contains(&single_elem)
            )
        )
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let common_elements: Vec<T> = self.items.iter()
            .flat_map(|vec| 
                vec.iter().filter(|item| other.contains(item)).map(|item| item.clone())
            )
            .collect();

        Self::new(common_elements.as_slice())
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let uncommon_elements: Vec<T> = self.items.iter()
            .flat_map(|vec| 
                vec.iter().filter(|item| !other.contains(item)).map(|item| item.clone())
            )
            .collect();

        Self::new(uncommon_elements.as_slice())
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self::new(Vec::from_iter(
            self.items.iter().flat_map(|vec| vec.into_iter().map(|item| *item)).chain(
                other.items.iter().flat_map(|vec| vec.into_iter().map(|item| *item))
            )
        ).as_slice())
    }
}
