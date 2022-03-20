use std::hash::Hash;

pub fn vecs() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    println!("{:?}", v);

    let v2 = vec![1, 2, 3, 4];
    println!("v2:{:?}, v2 first:{}", v2, &v2[0]);
    let _idx2 = &2;

    match v2.get(1) {
        Some(_idx2) => println!("second:{}", 2),
        None => println!("no data"),
    }

    for i in &v2 {
        println!("{}", i);
    }
}

pub fn map() {
    use std::collections::HashMap;

    let mut gem_map = HashMap::new();
    gem_map.insert("红宝石", 1);
    gem_map.insert("蓝宝石", 2);
    println!("{:?}", gem_map);

    let num :Option<&i32> = gem_map.get("红宝石");
    println!("{:?}", num);

    for (key, value) in gem_map {
        println!("key:{}, value:{}", key, value);
    }

    let team_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let team_map: HashMap<_, _> = team_list.into_iter().collect();

    println!("{:?}", team_map);
}
