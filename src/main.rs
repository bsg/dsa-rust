use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(5);

    for i in &mut list {
        *i = 0;
    }

    for i in list {
        println!("{}", i);
    }
}
