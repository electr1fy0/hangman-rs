struct Table {
    data: [char; 9],
}
impl Table {
    fn new() -> Self {
        let mut arr: [char; 9] = ['u'; 9];

        for i in &mut arr {
            *i = '-';
        }
        Table { data: arr }
    }
}
fn main() {
    let table = Table::new();
    let mut idx = 0;
    while idx < 9 {
        print!("{}   ", table.data[idx]);
        idx += 1;
        if idx % 3 == 0 && idx != 0 {
            println!();
        }
    }

    let cnt = 9;

    loop {
        println!()
    }
}

fn u8_input() {}
