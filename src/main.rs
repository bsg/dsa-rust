use singly_linked_list::List;

mod singly_linked_list;

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(5);

    for i in &mut list {
        *i = 0;
    }

    for i in list {
        println!("{}", i);
    }
}
