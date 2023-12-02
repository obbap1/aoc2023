use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day_one() -> u32 {
    let file = match File::open("src/input/one.txt") {
        Err(e) => panic!("{}",e),
        Ok(f) => f 
    };

    let buf_reader = BufReader::new(file);

    let mut result = 0;
    let empty_string: &'static str = "";

    for line in buf_reader.lines() {
        let curr_line = line.expect("unable to read line from buffer");
        let parts: Vec<String> = curr_line.split(empty_string).map(String::from).collect();
        let mut ss: [&str; 2] = [empty_string, empty_string];
        for i in 0..parts.len() {
            if ss[0] == empty_string && parts[i].parse::<u32>().is_ok(){
                ss[0] = &parts[i];
            }
        }

        for i in (0..parts.len()).rev() {
            if ss[1] == empty_string && parts[i].parse::<u32>().is_ok(){
                ss[1] = &parts[i];
            }
        }
        let s = format!("{}{}", ss[0], ss[1]).parse::<u32>().expect("");
        result += s;

    }

    result
}

pub fn day_one_b() -> u32 {
    let m = HashMap::from(
        [
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
            (String::from("four"), 4),
            (String::from("five"), 5),
            (String::from("six"), 6),
            (String::from("seven"), 7),
            (String::from("eight"), 8),
            (String::from("nine"), 9),
        ]
    );
    
    let file = match File::open("src/input/one.txt") {
        Err(e) => panic!("{}",e),
        Ok(f) => f 
    };

    let buf_reader = BufReader::new(file);

    let mut result = 0;
    let empty_string: &'static str = "";

    for line in buf_reader.lines() {
        let curr_line = line.expect("unable to read line from buffer");
        let parts: Vec<String> = curr_line.split(empty_string).filter(|&x| !x.is_empty()).map(String::from).collect();
        let mut ss: [String; 2] = [String::from(empty_string), String::from(empty_string)];
        let mut dd: [usize; 2] = [0, 0];

        for i in 0..parts.len() {
            if ss[0] == empty_string && parts[i].parse::<u32>().is_ok(){
                ss[0] = parts[i].clone();
                dd[0] = i;
            }
        }

        for i in (0..parts.len()).rev() {
            if ss[1] == empty_string && parts[i].parse::<u32>().is_ok(){
                ss[1] = parts[i].clone();
                dd[1] = i;
            }
        }

        for key in m.keys() {
            let cc: Vec<usize> = curr_line.match_indices(key).map(|(i, _)|i).collect(); 
            for i in cc {
                let kk = m.get(key).expect("").to_owned();
                if i < dd[0] {
                    dd[0] = i;
                    ss[0] = format!("{}",kk);  
                }
                if i > dd[1] {
                    dd[1] = i;
                    ss[1] = format!("{}",kk.clone());  
                }
            }
        }
        

        let s = format!("{}{}", ss[0], ss[1]).parse::<u32>().expect("");

        result += s;
    }
    result
}