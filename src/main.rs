use hello::twitter_utils::*;
fn main() {
    let project_dir = "/home/user/Documents/hello/files";
    //let tiny_data_file = format!("{}/twit.txt", project_dir);
    //get_top_users(&tiny_data_file, 5);
    let big_data_file = format!("{}/twitter-2010.txt", project_dir);
    get_top_users(&big_data_file, 7);
    //let medium_data_file = format!("{}/medium.txt", project_dir);
    //get_top_users(&medium_data_file, 2);
}
