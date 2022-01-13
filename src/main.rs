use std::collections::HashMap;

fn walk(categories: &HashMap<&str, &str>, child: &str, results: &mut Vec<String>) {

    let parent = categories[child];
    
    while parent != "" {
        println!("{:?}", results);
        results.push(parent.clone().to_string());
        walk(categories, parent, results);
    }
}

fn main() {
    let categories = HashMap::from([
        ("Insurance", "Housing"),
        ("CouncilTax", "Housing"),
        ("Maintenance", "Housing"),
        ("Energy", "Utility"),
        ("Internet", "Utility"),
        ("Phone", "Utility"),
        ("Housing", "Needs"),
        ("Utility", "Needs"),
        ("Needs", ""),
        ]);

    let mut results = Vec::new();
    walk(&categories, "Insurance", &mut results);
    
    println!("{:?}", results);
}