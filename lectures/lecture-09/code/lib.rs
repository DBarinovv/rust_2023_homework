// MPSC https://doc.rust-lang.org/std/sync/mpsc/index.html
// Mutex https://doc.rust-lang.org/std/sync/struct.Mutex.html
// Condvar https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html
// Deref Arc https://doc.rust-lang.org/std/sync/struct.Arc.html#deref-behavior

// Sync: send() can block. Lim capacity
// Mutex + Condvar + VecDeque
// Async: send() cannot block. Not lim capacity
// Mutex + Condvar + VecDeque
// Mutex + Condvar + LinkedList
// Atomic LinkedList
// Rendevouz: cap = 0
// Oneshot: any capacity. once call send()


use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // let mut inner = self.shared.inner.lock().unwrap();
        // inner.senders += 1;
        // drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let inner = self.shared.inner.lock().unwrap();
        // inner.senders -= 1;

        // let was_last = inner.senders == 0;
        let was_last = Arc::strong_count(&self.shared) == 2;
        drop(inner);

        if was_last {
            self.shared.avaliable.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);

        drop(inner);
        self.shared.avaliable.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue)
                    }

                    return Some(t);
                }
                None if Arc::strong_count(&self.shared) == 1 => return None,
                None => {
                    inner = self.shared.avaliable.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    avaliable: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            senders: 1,
        }),
        avaliable: Condvar::new(),
    };

    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver { shared, buffer: VecDeque::new() },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn tx_closed() {
        let (mut tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
}
