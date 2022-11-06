use std::{
    env, process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Not enought arguments");
        process::exit(1);
    }

    let number_of_queues: usize = args[1].parse().unwrap();

    let queue = Arc::new(Mutex::new(create_queue(number_of_queues)));

    // let queue1 = Arc::clone(&queue);
    let print_queue_arc = Arc::clone(&queue);

    for i in 0..number_of_queues {
        let create_queue_arc = Arc::clone(&queue);
        thread::spawn(move || loop {
            {
                let mut aux = create_queue_arc.lock().unwrap();
                aux[i].push('x');
            }
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_secs(rng.gen_range(1..5)));
        });

        let remove_queue_arc = Arc::clone(&queue);
        thread::spawn(move || loop {
            {
                let mut aux = remove_queue_arc.lock().unwrap();
                if !&aux.is_empty() {
                    aux[i].pop();
                }
            }
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_secs(rng.gen_range(1..5)));
        });
    }

    thread::spawn(move || loop {
        {
            let queue_guard = print_queue_arc.lock().unwrap();
            print_queue(&queue_guard);
        }
        thread::sleep(Duration::from_secs(1));
    });

    loop {}
}

fn get_queue_line(queue: &Vec<char>, i: usize) -> &str {
    if queue.len() >= i {
        return "|x| ";
    }
    return "| | ";
}

fn print_queue(supermarket: &Vec<Vec<char>>) {
    print!("\x1B[2J\x1B[1;1H");

    let mut header = String::new();
    for _ in 0..supermarket.len() {
        header.push_str("|_| ");
    }
    println!("{}", header);

    for i in 0..get_bigger_queue(supermarket) {
        let mut line = String::new();
        for queue in 0..supermarket.len() {
            line.push_str(get_queue_line(&supermarket[queue], i));
        }
        println!("{}", line);
    }
    println!("\n\n");
}

fn get_bigger_queue(supermarket: &Vec<Vec<char>>) -> usize {
    let mut size = 0;
    for queue in supermarket {
        if queue.len() > size {
            size = queue.len();
        }
    }
    size
}

fn create_queue(number_of_queues: usize) -> Vec<Vec<char>> {
    let mut supermarket = Vec::new();
    for _ in 0..number_of_queues {
        supermarket.push(Vec::<char>::new());
    }
    supermarket
}
