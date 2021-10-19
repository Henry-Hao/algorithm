/// Min-Heap is a Tree where the root node is always the smallest element in the three.
/// data is stored in Vec
/// Index:
/// current node: i
/// parent node: (i - 1) / 2
/// child node: i * 2 + 1, i * 2 + 2
/// 1. push: O(log(n))
/// 2. pop: O(log(n))
/// 3. is_empty: O(1)
/// 4. len: O(1)
#[derive(Default, Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                let result = Some(self.data.swap_remove(0));
                self.shift_down();
                result
            }
        }
    }

    /// Called after pop, this function will put the first element to the correct place
    fn shift_down(&mut self) {
        let mut i = 0;
        let len = self.data.len();
        while i < len {
            let left_idx = i * 2 + 1;
            let right_idx = i * 2 + 2;
            let mut swap_idx = i;
            if left_idx < len && self.data[left_idx] < self.data[i] {
                swap_idx = left_idx;
            }
            if right_idx < len && self.data[right_idx] < self.data[swap_idx] {
                swap_idx = right_idx;
            }
            if swap_idx == i {
                return;
            }
            self.data.swap(swap_idx, i);
            i = swap_idx;
        }
    }

    pub fn push(&mut self, ele: T) {
        self.data.push(ele);
        self.shift_up();
    }

    // Called after push, this function should put the last element to the correct place
    fn shift_up(&mut self) {
        let mut i = self.len() - 1;
        while i > 0 {
            let parent_idx = (i - 1) / 2;
            if self.data[parent_idx] < self.data[i] {
                return;
            }
            self.data.swap(parent_idx, i);
            i = parent_idx;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn test_new() {
        let heap: Heap<i32> = Heap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_len() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(1);
        heap.push(2);
        assert_eq!(heap.len(), 3);
    }

    #[test]
    fn test_pop() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);
        heap.push(7);
        heap.push(4);
        heap.push(3);
        heap.push(1);
        heap.push(9);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(7));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), None);
    }
}
