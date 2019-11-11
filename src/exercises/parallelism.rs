use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn spawn_threads(n: usize) {
    let mut handles = vec![];
    for num in 0..n {
        let handle = thread::spawn(move || {println!("Thread {} says hi c:", num);});
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}

pub fn do_calc(list: &Vec<isize>) -> isize {
    const MAX_N_THREADS: usize = 10;
    let (tx, rx) = mpsc::channel::<isize>();
    let mut handles = vec![];

    let n_threads = std::cmp::min(list.len() / 2, MAX_N_THREADS);

    for thread_id in 0..n_threads {
        let thread_tx = tx.clone();
        let slice = &list[thread_id..(thread_id + (list.len() / n_threads))];
        let mut copied = vec![];
        for i in slice.iter() {
            let _ = copied.push(*i);
        }
        let handle = thread::spawn(move || {
            let mut sum = 0isize;
            for i in copied.iter() {
                sum += *i;
            }
            let _ = thread_tx.send(sum);
        });
        handles.push(handle);
    }

    let mut threads_responded = 0;
    let mut final_value = 0isize;
    while threads_responded < n_threads {
        let res = rx.recv_timeout(Duration::from_millis(5));
        match res {
            Ok(r) => {
                final_value += r;
                threads_responded += 1;
            },
            Err(_) => panic!("A thread did not answer in time.")
        }
    }
    return final_value;

}

#[cfg(test)]
mod test {
}