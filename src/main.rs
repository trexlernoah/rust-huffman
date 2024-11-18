use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    p: usize,
    v: String,
}

fn create_parent(left: Node, right: Node, v: String, p: usize) -> Node {
    Node {
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        v,
        p,
    }
}

fn collect_hash_set(str: &String) -> HashMap<char, usize> {
    str.chars()
        .into_iter()
        .map(|c| (c, str.matches(c).count()))
        .collect()
}

fn sort_hash_map(map: HashMap<char, usize>) -> Vec<Node> {
    let mut v: Vec<Node> = map
        .into_iter()
        .map(|(v, p)| Node {
            left: None,
            right: None,
            v: v.to_string(),
            p,
        })
        .collect();
    v.sort_by(|a, b| b.p.cmp(&a.p));

    v
}

fn generate_dictionary(node: Node, code: String, dict: &mut HashMap<String, String>) {
    match node.left {
        Some(left_node) => generate_dictionary(*left_node, code.clone() + "0", dict),
        None => {
            dict.insert(node.v.clone(), code.clone());
        }
    };

    match node.right {
        Some(right_node) => generate_dictionary(*right_node, code.clone() + "1", dict),
        None => {
            dict.insert(node.v.clone(), code.clone());
        }
    }
}

fn convert_to_binary(str: String, dict: HashMap<String, String>) -> Vec<u8> {
    let mut res: String = String::from("");
    for char in str.chars() {
        res += dict.get(&char.to_string()).unwrap();
    }

    let mut vec: Vec<u8> = Vec::new();
    for byte in res.as_bytes() {
        println!("{byte}");
    }

    // res = code
    // .chars()
    // .into_iter()
    // .fold(res, |acc, c| (acc << 1) + (c as u128 - 0x30));

    vec
}

fn compress(str: String) -> String {
    let mut vec = sort_hash_map(collect_hash_set(&str));

    while vec.len() > 1 {
        let left = vec.pop().unwrap();
        let right = vec.pop().unwrap();
        let v = left.v.to_owned() + &right.v;
        let p = left.p + right.p;
        let internal_node = create_parent(left, right, v, p);

        // Insert in order
        let index = vec
            .binary_search_by(|node| (internal_node.p).cmp(&(node.p)))
            .unwrap_or_else(|i| i);
        vec.insert(index, internal_node);
    }

    assert!(vec.len() == 1);

    let mut dictionary = HashMap::new();
    generate_dictionary(vec.pop().unwrap(), String::from(""), &mut dictionary);
    println!("{:?}", dictionary);
    let res = convert_to_binary(str, dictionary);
    println!("{:?}", res);

    String::from("")
}

fn main() {
    let test_str = String::from("A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED");
    let res = compress(test_str);
    println!("{res}");
}
