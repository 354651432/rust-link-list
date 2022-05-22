use rust_demo::link::NodeList;

fn main() {
    let mut list = NodeList::new();

    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);
    list.push(50);
    list.push(60);
    list.push(70);

    let last = list.pop();
    println!("last is {:?}", last);

    let last = list.pop();
    println!("last is {:?}", last);

    let fist = list.shift();
    println!("first is {:?}", fist);

    list.unshift(11);
    for it in &list {
        println!("{it}")
    }

    let find = list.find(40);
    println!("value of find is {}", find.unwrap().data());

    list.insert(55, 0);
    let insert = list.insert(33, 3);
    println!("inserted value is {}", insert.unwrap().data());

    println!("after inserted");
    for it in &list {
        println!("{it}")
    }

    println!("idx 3 of value {}", list[3]);
    list[3] = 99;

    println!("after change by index 3");
    for it in list.iter() {
        println!("{it}")
    }

    for it in list.mut_iter() {
        *it += 200;
    }

    println!("after mut itered");
    for it in list.iter() {
        println!("{it}")
    }
}
