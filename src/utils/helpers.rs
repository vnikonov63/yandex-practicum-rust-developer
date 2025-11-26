pub fn greet() {
    println!("Hello world! from a spearate file!");
}

pub fn show_progress(current: u32, total: u32) {
    const WIDTH: u32 = 20;

    let percentage = current * 100 / total;
    let filled = percentage * WIDTH / 100;

    let mut bar = String::new();

    for i in 0..WIDTH {
        if i < filled {
            bar.push('█');
        } else {
            bar.push(' ');
        }
    }

    println!("[{}] {}%", bar, percentage);
}
