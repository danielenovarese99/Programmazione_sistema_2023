use std::sync::{Arc, Condvar, Mutex};
struct ContatoreBarriera {
    count: i32,
    generation_id: i32,
}
struct CyclicBarrier{
    busy: Mutex<ContatoreBarriera>,
    condvar : Condvar,
    n_threads : i32,
}

impl CyclicBarrier{
    pub fn new(n_threads: i32)->Self{
        CyclicBarrier{
            busy: Mutex::new(ContatoreBarriera{count: 0,generation_id: 0}),
            condvar: Condvar::new(),
            n_threads,
        }
    }

    pub fn wait(&self){
        let mut lock = self.busy.lock().unwrap();
        let local_gen = lock.generation_id;
        lock.count += 1;
        if lock.count < self.n_threads{
            // /// keep waiting
            //while local_gen == lock.generation_id{
            //    lock = self.condvar.wait(lock).unwrap();
           // }
            lock = self.condvar.wait(lock).unwrap();

            return;
        }
        else{
            lock.count = 0;
            //lock.generation_id = lock.generation_id.wrapping_add(1);
            lock.generation_id = lock.generation_id + 1;
            self.condvar.notify_all();
            return;
        }
    }
}


fn main() {
    println!("Hello, world!");

    let abarrrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 { /// spawn 3 threads
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}\n", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }

}
