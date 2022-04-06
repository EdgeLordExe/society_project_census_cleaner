use std::{collections::{HashMap, HashSet}, fs::File, io::{Read, Write}};

fn main() {
    let mut group_map : HashMap<String,String> = HashMap::new();
    
    let mut in_file = File::open("in.txt").unwrap();
    let mut out_file = File::create("out.txt").unwrap();
    let mut in_string = String::new();
    let mut ignore : HashSet<String> = HashSet::new();
    in_file.read_to_string(&mut in_string).unwrap();

    in_string.lines().for_each(|line| {
        identify(&line.to_string(), &mut out_file, &mut group_map, &mut ignore);
    });
}

fn identify(start_name: &String, out: &mut File ,  map: &mut HashMap<String,String>, ignore_set : &mut HashSet<String>) {
    if start_name.len() < 2{
        return;        
    }
    let name = start_name.trim().to_string().to_uppercase();    
    if ignore_set.contains(&name){
        return;
    }
    if map.contains_key(&name) {
        out.write(map.get(&name).unwrap().as_bytes()).unwrap();
        out.write(b"\n").unwrap();
        return;
    }
    println!("Add {} to database? type y to add with changes, a to add as is , i to ignore", name);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.starts_with("y") {
        println!("What is the name for the entry?");
        let mut group = String::new();
        std::io::stdin().read_line(&mut group).unwrap();
        map.insert(name.to_string(), group.trim().to_string());

        out.write(map.get(&name).unwrap().as_bytes()).unwrap();
        out.write(b"\n").unwrap();

    } else if input.starts_with("i") {
        ignore_set.insert(name.clone());
    } else if input.starts_with("a"){
        map.insert(name.to_string(), name.to_string());
        out.write(map.get(&name).unwrap().as_bytes()).unwrap();
        out.write(b"\n").unwrap();
    }
}
