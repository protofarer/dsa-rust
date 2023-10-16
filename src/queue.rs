#![allow(unused)]
// * fixed-size impl
// - PRO: 1. fast access via index O(1) 2. no pointers 3. simplest impl 4. predictable memory usage
// - CON: 1. resize operation  2. unused capacity is wasted memory

// * dynamic-size impl
// - PRO: 1. flexible
// - CON: 1. slower than fixed-size 2. larger memory footprint

// ? when shift head, set to None or just move index?

struct Queue<T, const N: usize> {
    data: [Option<T>; N],
    size: usize,
    head: usize,
    tail: usize, // Points to empty element
}

#[derive(PartialEq, Debug)]
enum QueueError {
    TooSmall,
    Underflow,
    Overflow,
}

impl<T: Clone + Copy + Default, const N: usize> Queue<T, N> {
    fn new() -> Result<Self, QueueError> {
        if N < 2 {
            eprintln!("Queue must have capacity of at least 2");
            return Err(QueueError::TooSmall);
        }
        Ok(Self {
            data: [None; N],
            size: 0,
            head: 0,
            tail: 0,
        })
    }
    // add item to queue
    pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
        // check if full
        if self.size() >= N {
            return Err(QueueError::Overflow);
        }
        self.data[self.tail] = Some(item);
        self.tail = self.add_index_delta(self.tail, 1);
        self.size += 1;
        Ok(())
    }

    // remove item from front of queue
    pub fn dequeue(&mut self) -> Result<Option<T>, QueueError> {
        // check if empty
        if self.size() == 0 {
            return Err(QueueError::Underflow);
        }
        let item = self.data[self.head];
        self.head = self.add_index_delta(self.head, 1);
        self.size -= 1;
        Ok(item)
    }

    pub fn size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn add_index_delta(&self, i: usize, di: usize) -> usize {
        (i + (di % N)) % N
    }
    pub fn capacity(&self) -> usize {
        N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_core() {
        let mut q: Queue<i32, 2> = Queue::new().unwrap();

        assert_eq!(q.size(), 0);
        assert_eq!(q.is_empty(), true);
        assert_eq!(q.dequeue(), Err(QueueError::Underflow));
        assert_eq!(q.capacity(), 2);

        assert_eq!(q.enqueue(12), Ok(()));
        assert_eq!(q.size(), 1);
        assert_eq!(q.is_empty(), false);
        assert_eq!(q.capacity(), 2);

        assert_eq!(q.enqueue(24), Ok(()));
        assert_eq!(q.size(), 2);

        assert_eq!(q.enqueue(108), Err(QueueError::Overflow));

        assert_eq!(q.dequeue(), Ok(Some(12)));
        assert_eq!(q.dequeue(), Ok(Some(24)));
        assert_eq!(q.dequeue(), Err(QueueError::Underflow));
        assert_eq!(q.is_empty(), true);
    }
}
