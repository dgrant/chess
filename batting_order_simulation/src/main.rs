extern crate permutohedron;
extern crate rand;
extern crate statistical;

//use permutohedron::LexicalPermutation;
use std::cmp::Ordering;
//use permutohedron::heap_recursive;
use rand::{thread_rng, random};
use rand::seq::SliceRandom;
use std::collections::LinkedList;

#[derive(Debug)]
#[derive(Eq)]
struct Player {
    name: String,
    obs: u32,
    hits_doubles: bool,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.obs.cmp(&other.obs)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.obs == other.obs
    }
}

fn build_player(name: String, obs: u32, hits_doubles: bool) -> Player {
    Player {
        name,
        obs,
        hits_doubles,
    }
}

fn evaluate_order_python(players: &Box<[Player]>,
                         num_innings: usize,
                         run_limits: [u32; 5],
                         num_batters: usize) -> u32 {
    let mut order_position: usize = 0;
    let mut runs_scored: u32 = 0;
    let mut inning: usize = 0;
    let mut bases: LinkedList<bool> = LinkedList::new();
    while inning < num_innings {
        bases.clear();
//        println!("Inning: {:?}", inning);
        let mut num_outs: u32 = 0;
        let mut runs_scored_this_inning: u32 = 0;
        let runs_limit_for_inning: u32 = run_limits[inning];
        while num_outs < 3 && runs_scored_this_inning < runs_limit_for_inning {
            order_position += 1;
            order_position = order_position % num_batters;
            let player = &players[order_position];
//            println!("Player: {:?}", player);
            let obs_int: u32 = player.obs;
            let random_number: f64 = random();
//            println!("{:?}", random_number);
            if ((random_number * 1000__f64) as u32) < obs_int {
//                println!("*** HIT!");
                bases.push_front(true);
                if player.hits_doubles && random::<f64>() < 0.4__f64 {
                    bases.push_front(false);
                }
            } else {
//                println!("*** OUT!");
                num_outs += 1
            }
            while bases.len() > 3 {
                if bases.pop_back().unwrap() {
                    runs_scored_this_inning += 1;
//                    println!("*** Run scored!!");
                }
            }
        }
//        println!("Runs scored this inning: {:?}", runs_scored_this_inning);
        runs_scored += runs_scored_this_inning;
        inning += 1;
    }
    return runs_scored;
}


fn main() {
    let obs_stats = [
        ("Naden", 0.833, true),
        ("Isaiah", 0.750, true),
        ("Claire", 0.722, true),
        ("Caleb", 0.618, false),
        ("Sam", 0.528, false),
        ("Piper", 0.576, false),
        ("William", 0.400, false),
        ("Felix", 0.267, false),
        ("Casey", 0.414, false),
        ("Zayne", 0.345, false),
//        ("Jodhyn", 0.250, false),
        ("Aiden", 0.367, false),
    ];
    let mut players = Vec::new();
    let mut index = 0;
    while index < obs_stats.len() {
        let player = build_player(String::from(obs_stats[index].0),
                                  (obs_stats[index].1 * 1000__f64).round() as u32,
                            obs_stats[index].2);
        players.push(player);
        index += 1;
    }
    let mut boxed_players: Box<[Player]> = players.into_boxed_slice();
    println!("{:?}", boxed_players);
    let mut rng = thread_rng();
    let mut best_runs_scored: f32 = 0__f32;
    let mut best_stddev: f32;
    let mut number_orders_tested = 0;
    loop {
        const NUM_SAMPLES: usize = 1000;
        let mut runs_array: [f32; NUM_SAMPLES] = [0__f32; NUM_SAMPLES];
        let mut sample_counter: usize = 0;
        while sample_counter < NUM_SAMPLES {
            let runs_scored: f32 = evaluate_order_python(&boxed_players,
                                                         5,
                                                         [2, 2, 4, 4, 99],
                                                         obs_stats.len()) as f32;
            runs_array[sample_counter] = runs_scored;
            sample_counter += 1;
        }
        let mean_runs_scored: f32 = statistical::mean(&runs_array);
        let stddev = statistical::standard_deviation(&runs_array, None);
        if mean_runs_scored > best_runs_scored {
            best_runs_scored = mean_runs_scored;
            best_stddev = stddev;
            println!("{:?}", best_runs_scored);
            println!("{:?}", best_stddev);
            println!("{:?}", &boxed_players);
        }
        number_orders_tested += 1;
        if number_orders_tested % 1000000 == 0 {
            println!("Number of orders tested: {:?}", number_orders_tested);
        }
//        if !boxed_players.next_permutation() {
//            break;
//        }
        boxed_players.shuffle(&mut rng);
//        break;
    }
//    println!("{:?}", boxed_players);
}
