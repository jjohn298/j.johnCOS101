use std::io::Write;

fn main() {

    let lager = "Lager-\n33 Export\nDesperados\nGoldberg\nHeineken\nStar";
    let stout = "Stout-\nLegend\nTurbo King\nWilliams";
    let non_alcoholic = "Non-Alcoholic-\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz";

    let mut file = std::fs::File::create("NigerianBreweries.csv").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
        file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("\nData written to file.");

}
