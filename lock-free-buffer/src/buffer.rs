use std::{
    hint,
    mem::ManuallyDrop,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

/// Thread-Safe, Append Only and Fixed Sized Buffer
pub struct SafeBuffer<T> {
    // how big of memory we want to allocate at once
    capacity: usize,
    // how many elements are there in the buffer
    len: AtomicUsize,
    // at which location we are writing the data
    writing_len: AtomicUsize,
    // the actual buffer
    buffer: AtomicPtr<T>,
}

impl<T> SafeBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer = Vec::with_capacity(capacity);
        let buffer = AtomicPtr::new(ManuallyDrop::new(buffer).as_mut_ptr());
        Self {
            capacity,
            len: AtomicUsize::new(0),
            writing_len: AtomicUsize::new(0),
            buffer,
        }
    }

    /// Append element at the last of the buffer
    pub fn append(&self, elem: T) {
        let writing_index = self.writing_len.fetch_add(1, Ordering::SeqCst);
        if writing_index >= self.capacity {
            panic!("index out of bound: {}", writing_index);
        }

        // SAFETY: We have made a checked, index must be smaller than capacity
        let writing_location = unsafe { self.buffer.load(Ordering::Relaxed).add(writing_index) };
        // SAFETY: We have made sure memory has allocated, and more than one thread will
        // never writes to this location
        unsafe { writing_location.write(elem) };

        // Now we need to increment the len
        // More than one thread try increment the len at the same time(concurrent writes),
        // We have wo wait until other concurrent threads have not finished their updates.
        loop {
            match self.len.compare_exchange(
                writing_index,     // expected
                writing_index + 1, // new value
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_idx) => break,
                Err(_) => {
                    hint::spin_loop();
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let len = self.len.load(Ordering::SeqCst);
        if index >= len {
            return None;
        }
        let read_location = unsafe { self.buffer.load(Ordering::Relaxed).add(index) };
        unsafe { read_location.as_ref() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_test() {
        let buffer = SafeBuffer::new(10000000);
        std::thread::scope(|s| {
            let mut handles = vec![];
            for _ in 0..10 {
                let h = s.spawn(|| {
                    for _ in 0..1000000 {
                        buffer.append(1);
                    }
                });
                handles.push(h);
            }

            // wait for all threads to join
            for h in handles {
                h.join().unwrap();
            }

            let mut sum = 0;
            for idx in 0..10000000 {
                sum += buffer.get(idx).unwrap();
            }

            println!("sum: {}", sum);
            assert_eq!(10 * 1000000, sum);
        })
    }
}
