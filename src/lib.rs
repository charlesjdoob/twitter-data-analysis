pub mod twitter_utils {
    use std::io::BufWriter;
    use std::iter::FromIterator;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::collections::HashMap;
    use std::time::Instant;
    const TOP_USERS_COEFFICIENT: f64 = 0.10;
    const TOP_USERS_FILE: &str = "top-users-result";
    const PATH: &str = "/home/user/Documents/hello/files/";
    pub fn get_top_users(input_file_name: &str, num_iterations: u32) {
        let mut now = Instant::now();
        let mut curr_iteration = 0;
        let mut pruned_file_name: String;
        let mut curr_input_file: String =(*input_file_name).to_string();
        let mut top_users_file_name: String;
        while curr_iteration < num_iterations {
            let mut result: Vec<(&u64, &u64)>;
            let top_users: HashMap<u64, u64> = count_followers(&curr_input_file);
            pruned_file_name = format!("{}.pruned{}", curr_input_file, curr_iteration);
            prune_data(&curr_input_file, &pruned_file_name, &top_users); 
            curr_input_file = pruned_file_name; 
            println!("Time for iteration {}: {:?}", curr_iteration, now.elapsed());
            now = Instant::now();
            curr_iteration += 1;
            top_users_file_name = format!("{}{}{}.txt", PATH, TOP_USERS_FILE, curr_iteration);
            let path = Path::new(&top_users_file_name);
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}", why),
                Ok(file) => BufWriter::new(file),
            };
            result = Vec::from_iter(top_users.iter());
            result.sort_by(|a, b| b.1.cmp(&a.1));
            match file.write_all(format!("{:?}", result).as_bytes()) {
                Err(why) => panic!("couldn't write to {}", why),
                Ok(_) => ()
            }
        }
    }
    pub fn count_followers(input_file_name: &str) -> HashMap<u64, u64> {
        let mut lines_read = 0;
        let mut followers = HashMap::<u64, u64>::new();
        
        if let Ok(lines) = read_lines(input_file_name) {
            // Go through twitter data file and make map where the key is the ID of the user being
            // followed and the value is the number of total followers.
            for line in lines {
                if let Ok(ip) = line {
                    let user_id : u64 = match ip.split_whitespace().next() {
                        Some(id) => id.parse::<u64>().unwrap(),
                        None => 0
                    };
                    
                    let num_followers: u64 = match followers.get(&user_id) {
                        Some(curr_num_followers) =>  *curr_num_followers + 1,
                        None => 1 
                    };
                    followers.insert(user_id, num_followers);
                    
                    if lines_read % 100000000 == 0 {
                        println!("read {} lines!", lines_read);
                    }
                    lines_read += 1;
                }
            }
        }
        //Write resulting user id and number of follower pairs to a new file sorted in descending
        //order based on number of followers
        //Create a vec of tuples so I can sort all the stuff because I am lazy and stupuid
        let mut followers_vec = Vec::from_iter(followers.iter());
        followers_vec.sort_by(|a, b| b.1.cmp(&a.1));
        followers_vec.truncate(((followers_vec.len() as f64) * TOP_USERS_COEFFICIENT).round() as usize);
        let result = followers_vec.into_iter().map(|a| (*a.0, *a.1)).collect(); 
        result
    }
    
    // Create new file from previous input data which only contains lines where both users are in
    // the users_to_keep map
    fn prune_data(input_file_name: &str, output_file_name: &str, users_to_keep: &HashMap<u64, u64>) { 
        let mut now = Instant::now();
        let mut lines_read = 0;

        if let Ok(lines) = read_lines(input_file_name) {
            let mut file = match File::create(&Path::new(&output_file_name)) {
                Err(why) => panic!("couldn't create {}", why),
                Ok(file) => BufWriter::new(file),
            };
            let mut prev_second_user: u64 = u64::MAX;
            let mut keep_second_user: bool = false;
            for line in lines {
                if let Ok(ip) = line {
                    let mut line_iter = ip.split_whitespace();
                    let first_user_id_unparsed = match line_iter.next() {
                        Some(id) => id,
                        None => ""
                    };
                    let second_user_id : u64 = match line_iter.next() {
                        Some(id) => id.parse::<u64>().unwrap(),
                        None => 0
                    };
                    if second_user_id != prev_second_user {
                        prev_second_user = second_user_id;
                        keep_second_user = match users_to_keep.get(&second_user_id) {
                            Some(_) => true,
                            None => false
                        };
                    }
                    let mut keep_first_user: bool = false;
                    if keep_second_user {
                        let first_user_id = first_user_id_unparsed.parse::<u64>().unwrap();
                        keep_first_user = match users_to_keep.get(&first_user_id) {
                            Some(_) => true,
                            None => false
                        };
                    }
                    
                    if keep_first_user && keep_second_user { 
                        match file.write_all(format!("{}\n", ip).as_bytes()) {
                            Err(why) => panic!("couldn't write to {}", why),
                            Ok(_) => ()
                        }
                    }
                }
                if lines_read % 100000000 == 0 {
                    println!("Read {} lines in {:?}", lines_read, now.elapsed());
                    now = Instant::now();
                }

                    lines_read += 1;
            }
        }
    }
    // Helpers
    
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = match File::open(filename) {
            Err(why) => panic!("couldn't open {}", why),
            Ok(file) => file,
        };
        Ok(io::BufReader::new(file).lines())
    }
}




