pub struct Node<T> {
    pub item: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut length = 0;

        let mut current = &self.head;
        while let Some(curr_node) = current {
            length += 1;
            current = &curr_node.next;
        }

        length
    }

    pub fn push(&mut self, element: T) {
        let next_node = self.head.take();

        self.head = Some(Box::new(Node {
            item: element,
            next: next_node,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take();

        match node {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();

        let mut cur_node = self.head;
        while let Some(node) = cur_node {
            list.push(node.item);
            cur_node = node.next;
        }

        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        SimpleLinkedList {
            head: iter.into_iter().fold(None, |acc, elem| {
                Some(Box::new(Node {
                    item: elem,
                    next: acc,
                }))
            }),
        }
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();

        while !linked_list.is_empty() {
            if let Some(val) = linked_list.pop() {
                result.insert(0, val);
            }
        }
        result
    }
}
