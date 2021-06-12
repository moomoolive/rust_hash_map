mod data;

use data::{ HashMap };

fn main() {
    let mut map: HashMap<i32> = HashMap::default();
    let key = "bye";
    map.insert(key, 32);
    println!("total map b4: {:?}\n", map);
    println!("map val after insertion: {:?}\n\n", map.get(key));
    let rmved = map.remove(key);
    println!("removed value: {:?}\n\n", rmved);
    println!("map val after removal: {:?}", map.get(key));
    println!("total map after: {:?}", map);
}