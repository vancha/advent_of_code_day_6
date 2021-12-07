///the struct that will hold all data for this assignment
#[derive(Debug)]
struct Lanterns {
    ///the list that holds all the fishes
    fish: Vec<u128>,
    ///an i32 to indicate which day corresponds to the fishes in the list
    day:i32,
}

impl Lanterns {
    ///builder pattern for above struct, lets you create new instance of struct with
    ///```let l = Lanterns::new();``` 
    fn new(input: Vec<u128>)->Self {
        Lanterns { fish: input, day: 0 }
    }
    ///count the number of zeros in the list of fishes
    fn count_zeros(&self)->u128 {
        self.fish.iter().filter(|v| *v == &0).count() as u128
    }
    ///advance the list of fishes to the next day, and increment the ```day``` variable in the list
    fn next_day(&mut self) {
        //count the fishes that are ready to give birth (which are represented by zero's)
        let mut baby_fishes = (0..self.count_zeros()).map(|_|8).collect::<Vec<u128>>();
        //go over all the other fishes in the fish list
        for fish_timer in self.fish.iter_mut() {
            //for every fish that's not 0,
            if *fish_timer != 0 {
                //decrease it's counter
                *fish_timer -= 1;
            //for every fish that *is* 0
            } else {
                //set it's fishy timer to 6
                *fish_timer = 6;
            }
        }
        //introduce the baby fish to the fish list
        self.fish.append(&mut baby_fishes);
        //increment the day
        self.day += 1;
    }
    fn get_day(&self) -> i32 {
        self.day
    } 
}

fn main() {
    let input:Vec<u128> = std::fs::read_to_string("/home/vancha/Warpinator/aoc_6/input").unwrap().trim().split(",").map(|v|v.parse::<u128>().unwrap()).collect::<Vec<u128>>();
    //create a new fish struct with the initial fish as a starting point.
    let mut shoal = Lanterns::new(input);
    //until we reached day 80
    while &shoal.get_day() < &80 {
        //advance a day
        shoal.next_day();
    }
    //show how many fish there are when we reached day 80
    println!("len: {}",shoal.fish.len());




    //this doesn´t work for part two.. sadly enough
    //for this i must use stolen ideas

    let mut hashbrown = std::collections::HashMap::new();
    hashbrown.insert(1,1);

}
