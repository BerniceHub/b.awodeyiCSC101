use std::io::Write;

fn main() {

    let welcome = "Welcome to Nigerian Brewery Limited";
    let lager = vec!["\nLAGER: ","\n  33 Export ", "\n  Desperados ", "\n  Goldberg ", "\n  Gulder ", "\n  Heineken ", "\n  Star "];
    let stout = vec!["\nSTOUT: ","\n  Legend ", "\n  Turbo King ", "\n  Williams "];
    let non_alcoholics = vec!["\nNON-ALCOHOLICS: ","\n  Maltina ", "\n  Amstel Malta ", "\n  Malta Gold ", "\n  Fayrouz "];

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all(welcome.as_bytes()).expect("write failed");
    file.write_all("\nThe high-quality categories of drinks includes the following:".as_bytes()).expect("write failed");
    for i in 0..7 {
    file.write_all(lager[i].as_bytes()).expect("write failed");
}
    for j in 0..4 {
    file.write_all(stout[j].as_bytes()).expect("write failed");
}
    for k in 0..5 {
    file.write_all(non_alcoholics[k].as_bytes()).expect("write failed");
}
    println!("File is created");

}
