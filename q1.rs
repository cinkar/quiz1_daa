use std::io;

struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    fn heapify_down(&mut self, i: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;

        if left < self.heap.len() && self.heap[left] < self.heap[smallest] {
            smallest = left;
        }
        if right < self.heap.len() && self.heap[right] < self.heap[smallest] {
            smallest = right;
        }

        if smallest != i {
            self.heap.swap(i, smallest);
            self.heapify_down(smallest);
        }
    }

    fn heapify_up(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let parent = (i - 1) / 2;
        if self.heap[i] < self.heap[parent] {
            self.heap.swap(i, parent);
            self.heapify_up(parent);
        }
    }

    fn heap_insert(&mut self) {
        let mut input = String::new();
        println!("Input banyaknya data: ");
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        for i in 0..n {
            input.clear();
            println!("Angka ke-{}: ", i + 1);
            io::stdin().read_line(&mut input).unwrap();
            let item: i32 = input.trim().parse().unwrap();
            self.heap.push(item);
            self.heapify_up(self.heap.len() - 1);
        }
    }

    fn get_min(&self) -> Option<i32> {
        self.heap.first().copied()
    }

    fn print_heap(&self) {
        print!("Heap: ");
        for &data in &self.heap {
            print!("{} ", data);
        }
        println!();
    }
}

fn main() {
    let mut heap = MinHeap::new();
    heap.heap_insert();
    heap.print_heap();
    if let Some(min) = heap.get_min() {
        println!("Parent heap (Min): {}", min);
    } else {
        println!("Heap is empty");
    }
}
