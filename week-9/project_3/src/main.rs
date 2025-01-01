use std::fs::File;
use std::io::Write;

fn main() {
    let sn = [1,2,3,4,5];
    let headings = ["S/N","  NAME OF COMMISSIONER","        MINISTRY","           GEOPOLITICAL ZONE"];
    let names = ["   Aigbogun Alamba Daudu","   Murtala Afeez Bendu","   Okorocha Calistus Ogbona","   Adewale Jimoh Akanbi","   Osazuwa Faith Etieye"];
    let ministry = ["     Internal Affairs","       Justice","  Defense","      Power & Steel","      Petroleum"];
    let geo_zone = [" South West","          North East","          South South","    South West","        South East"];

    let mut file = std::fs::File::create("minister.txt").expect("create failed");
    for i in 0..headings.len() {
        file.write_all(headings[i].as_bytes()).expect("write failed");
    }
    for i in 0..names.len() {
        write!(file,"\n{} {}  {}  {}",sn[i],names[i],ministry[i],geo_zone[i]);
    }
    println!("File created successfully");
}

    

