#![allow(dead_code)]
#![allow(unused_assignments)]

use std::io;
use rand::Rng;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;


fn main() {
//    func2()
    // test2
    println!("{}", first_fay(&mut String::from("first")));
    println!("{}", first_fay(&mut String::from("apple")));

    //test3
    third();
//    func_name();
}

// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取
// 一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
fn third() {
    // crete map and vector
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
//    let mut cmd = String::new();
    loop {
        print!(">>");
        io::stdout().flush().unwrap();
        let mut cs = String::from("");
        io::stdin().read_line(&mut cs).expect("Failed to read number");
        if cs.trim().eq("exit") {
            break;
        }
        let mut words = cs.trim().split_whitespace();
        let fi = words.next();
        if fi == Some("Add") {
            let value = words.next().unwrap().to_string();
            let _to = words.next().unwrap();
            let key = words.next().unwrap().to_string();
            if map.contains_key(&key) {
                map.get_mut(&key).unwrap().push(value);
            } else {
                let mut vec = Vec::new();
                vec.push(value);
                map.insert(key, vec);
            }
        } else {
            match map.get(fi.unwrap()) {
                Some(value) => {
                    for val in value {
                        print!("{} ", val);
                    }
                    print!("\n");
                    io::stdout().flush().unwrap();
                }
                None => {
                    println!("no company found!");
                }
            }
        }
    }
}

//将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会
// 变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
fn first_fay(s: &mut String) -> &String {
    let first = s.remove(0);
    let ao = "aeiouAEIOU";
    if ao.chars().find(|&x| x == first).is_some() {
        s.insert(0, first);
        s.push_str("-hay");
    } else {
        s.push('-');
        s.push(first);
        s.push_str("ay");
    }
    s
}

fn func2() {
    let mut v = vec![1, 2, 3, 4, 5, 9, 7, 0, 9];
    let mut sum = 0;
    for value in &v {
        sum += *value;
    }
    println!("sum is : {}", sum);
    println!("avg is : {}", sum / v.len());
    v.sort();
    for value in &v {
        print!("{} ", value)
    }
    println!();
    println!("mid is : {}", v.get(v.len() / 2).unwrap());
    let mut map = HashMap::new();
    let mut most = 0;
    let mut most_sum = 0;
    for value in &v {
        let account = map.entry(value).or_insert(0);
        *account += 1;
        if *account > most_sum {
            most_sum = *account;
            most = *value;
        }
    }
    println!("{:?}", map);
    println!("{}", most);
}


fn func_name() {
    println!("Guess the number!");
    println!("Please input the number");
    let rand_n: i32 = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    loop {
        guess = String::from("");
        io::stdin().read_line(&mut guess).expect("Failed to read number");
        println!("You guess is : {}", guess.trim());
        let secret_number: i32 = match guess.trim().parse() {
            Ok(t) => { t }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        match rand_n.cmp(&secret_number) {
            Ordering::Less => println!("Your number is greater then the number."),
            Ordering::Greater => println!("Your number is less then the number."),
            Ordering::Equal => {
                println!("Your number is Equal to the number.");
                break;
            }
        }
    }
}