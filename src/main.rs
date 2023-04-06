use std::sync::Arc;
use std::thread;

fn main() {
    let arr = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let num_threads = 3;
    let chunk_size = arr.len() / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        let chunk_start = i * chunk_size;
        let chunk_end = if i == num_threads - 1 {
            arr.len()
        } else {
            (i + 1) * chunk_size
        };
        let chunk = Arc::clone(&arr);
        let handle = thread::spawn(move || {
            let slice = &chunk[chunk_start..chunk_end];
            let sum: i32 = slice.iter().sum();
            println!("This thread is handling:");
            for number in slice.iter() {
                print!("{}", number);
            }
            println!();

            sum
        });
        handles.push(handle);
    }

    let sum: i32 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    println!("The sum is: {}", sum);
}