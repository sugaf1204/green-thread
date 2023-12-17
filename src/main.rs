mod green;

// thread_id: 1
fn mash() {
    unsafe {
        green::spawn(ortega, 2 * 1024 * 1024);
        for _ in 0..10 {
            println!("Mash!");
            green::send(2, 1);
            let msg = green::recv(1);
            println!("Mash received: {}", msg);
        }
    }
}

// thread_id: 2
fn ortega() {
    for _ in 0..10 {
        println!("Ortega!");
        green::send(3, 2);
        let msg = green::recv(2);
        println!("Ortega received: {}", msg);
    }
}

// thread_id: 3
fn gaia() {
    unsafe {
        green::spawn(mash, 2 * 1024 * 1024);
        for _ in 0..10 {
            println!("Gaia!");
            green::send(1, 3);
            let msg = green::recv(3);
            println!("Gaia received: {}", msg);
        }
    }
}

fn main() {
    green::spawn_from_main(gaia, 2 * 1024 * 1024);
}
