use std::io::Read;
use std::io::Write;

fn main() {
    let headings = ["Student Name", "    Matric. Number", "    Department", "    Level"];
    let rows = [["\nOluchi Mordi","     ACC10211111","      Accounting","     300"],
               ["\nAdams Aliyu","      ECO10110101","      Economics","      100"],
               ["\nShania Bolade","    CSC10328828","      Computer","       200"],
               ["\nAdekunle Gold","    EEE11020202","      Electrical","     200"],
               ["\nBlanca Edemoh","    MEE10202001","      Mechanical","     100"]];

    let mut file = std::fs::File::create("students.txt").expect("create failed");
    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    for i in 0..4 {
        file.write_all(headings[i].as_bytes()).expect("write failed");
    }
    
    for row in &rows {
        for y in 0..4 {
            file.write(row[y].as_bytes()).expect("write failed");
        }
    }
    

    let mut open_file = std::fs::File::open("students.txt").unwrap();
    let mut contents = String::new();
    open_file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
