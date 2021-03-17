use scale::{Encode, Decode};
use serde_json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
struct Node {
    id: u32,
    age: u64,
}

#[derive(Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
struct Stt {
    str: String,
}

#[test]
pub fn get_test() {
    // methodName_len + methodName + param1_len + param1
    let uu: u32 = 111;
    let uu_encode = uu.encode();
    println!("{:?}", uu_encode);

    let mut str1 = "1this is str1".to_string().encode();
    let mut str2 = "1this is str2".to_string().encode();
    println!("{:?}", str1);
    println!("{:?}", str2);
    str1.append(&mut str2);
    let i2 = &mut &str1[..];
    let r1 = <String as scale::Decode>::decode(i2);
    let r2 = <String as scale::Decode>::decode(i2);
    // println!("{:?}", r1.unwrap());
    println!("{:?}", r2.unwrap());
    match r1.unwrap().as_str() {
        "1this is str1" => { println!("{}", "rrr") }
        _ => { println!("error") }
    }
    println!("-[-p-----------");

    let node = Node {
        id: 100,
        age: 20,
    };
    let st = Stt {
        str: "111000fasfas".parse().unwrap(),
    };
    let mut encoded = node.encode();
    let mut encoded_str = st.encode();
    let b2 = serde_json::to_vec(&node).unwrap();
    println!("{:?}", encoded);
    println!("{:?}", encoded_str);
    encoded.append(&mut encoded_str);
    let input = &mut &encoded[..];
    let res = <Node as scale::Decode>::decode(input);
    let res2 = <Stt as scale::Decode>::decode(input);
    println!("{}", res.unwrap().age);
    match res2 {
        Ok(res) => {
            println!("{}", res.str);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    // println!("{}", res2.unwrap().str);
}
