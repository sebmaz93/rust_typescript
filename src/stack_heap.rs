#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all_items(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut item = Item { count: 1 };
    println!("{:?}", item);

    add_one(&mut item);
    println!("{:?}", item);

    let mut items: Vec<Item> = vec![Item { count: 2 }];
    let first = items.first_mut();
    println!("{:?}", first);

    print_all_items(&items);
    println!("{:?}", first);
}
