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

fn collect_hash_set(str: String) -> HashMap<char, usize> {
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

fn compress(str: String) -> String {
    let mut vec = sort_hash_map(collect_hash_set(str));

    while vec.len() > 1 {
        let left = vec.pop().unwrap();
        let right = vec.pop().unwrap();
        let v = left.v.to_owned() + &right.v;
        let p = left.p + right.p;
        let internal_node = create_parent(left, right, v, p);

        // Insert in order
        let index = vec
            .binary_search_by(|node| (internal_node.p).cmp(&(node.p)))
            .unwrap_or_else(|e| e);
        vec.insert(index, internal_node);
    }

    String::from("")
}

fn main() {
    let test_str = String::from("A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED");
    let res = compress(test_str);
    println!("{res}");
}
