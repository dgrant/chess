extern crate permutohedron;
extern crate rand;
extern crate statistical;

//use permutohedron::LexicalPermutation;
use std::cmp::Ordering;
//use permutohedron::heap_recursive;
use rand::thread_rng;
use rand::seq::SliceRandom;

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
    while inning < num_innings {
        let mut num_on_base: u32 = 0;
        let mut num_outs: u32 = 0;
        let runs_limit_for_inning: u32 = run_limits[inning];
        while num_outs < 3 && runs_scored < runs_limit_for_inning {
            order_position += 1;
            order_position = order_position % num_batters;
            let obs_int: u32 = players[order_position].obs;
            let random_number: f64 = rand::random();
            if ((random_number * 1000__f64) as u32) < obs_int {
                num_on_base += 1;
                if num_on_base > 3 {
                    runs_scored += 1;
                }
            } else {
                num_outs += 1
            }
        }
        inning += 1;
    }
    return runs_scored;
}


fn main() {
    let obs_stats = [
        ("Felix", 0.259),
        ("Isaiah", 0.774),
        ("Caleb", 0.586),
        ("Claire", 0.710),
        ("Sam", 0.581),
        ("Piper", 0.571),
        ("William", 0.419),
        ("Casey", 0.360),
        ("Zayne", 0.320),
        ("Jodhyn", 0.286),
        ("Naden", 0.871),
        ("Aiden", 0.333),
    ];
    let mut players = Vec::new();
    let mut index = 0;
    while index < obs_stats.len() {
        let player = build_player(String::from(obs_stats[index].0),
                                  (obs_stats[index].1 * 1000__f64).round() as u32,
                                  false);
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
    }
//    println!("{:?}", boxed_players);
}
