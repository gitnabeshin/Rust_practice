use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("caluculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn gen_workout(intensity: u32, random_num: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );

        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_num == 3 {
            println!("Take a break today. Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} munutes!",
                expensive_result
            );
        }
    }
}

fn gen_workout_closure(intensity: u32, random_num: u32) {

    // closure starts from |num|
    let expensive_closure = |num:u32| -> u32 {
        println!("caluculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_num == 3 {
            println!("Take a break today. Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} munutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {
    let sim_user_value = 10;
    let sim_random_num = 7;
    // gen_workout(sim_user_value, sim_random_num);
    gen_workout_closure(sim_user_value, sim_random_num);
}
