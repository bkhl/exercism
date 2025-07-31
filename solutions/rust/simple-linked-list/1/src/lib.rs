pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    tail: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        let mut c = 1;
        let mut tail = &self.tail;
        while let &Some(ref boxed_node) = tail {
            c += 1;
            tail = &boxed_node.tail;
        }
        c
    }

    fn peek(&self) -> &T {
        &self.value
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            &None => 0,
            &Some(ref boxed_node) => boxed_node.len(),
        }
    }

    pub fn push(&mut self, element: T) {
        let new_head = Box::new(Node {
            value: element,
            tail: std::mem::replace(&mut self.head, None),
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            Some(boxed_node) => {
                let node = *boxed_node;
                self.head = node.tail;
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref boxed_node) => Some(boxed_node.peek()),
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        match self.head {
            Some(ref boxed_node) => {
                let mut node = boxed_node;
                loop {
                    list.push(node.value.clone());
                    match node.tail {
                        Some(ref boxed_node) => {
                            node = boxed_node;
                        }
                        None => break,
                    }
                }
            }
            None => {}
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = Self::new();
        for element in item.iter() {
            list.push(element.clone());
        }
        list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        match self.head {
            Some(ref boxed_node) => {
                let mut node = boxed_node;
                loop {
                    vec.push(node.value.clone());
                    match node.tail {
                        Some(ref boxed_node) => {
                            node = boxed_node;
                        }
                        None => break,
                    }
                }
            }
            None => {}
        }
        vec.reverse();
        vec
    }
}