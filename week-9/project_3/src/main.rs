use std::io::Write;

fn main() {
    let commissioner1 = vec!["Aigbogun Alamba Dauda","Internal Affairs","South west"];
    let commissioner2 = vec!["Murtala Afeez Bendu","Justice","North East"];
    let commissioner3 = vec!["Okorocha Calistus Ogbona","Defense","South south"];
    let commissioner4 = vec!["Adewale Jimoh Akanbi","Power & steel","South west"];
    let commissioner5 = vec!["Osazuwa Faith Etieye","Petroleum","South east"];

    let header = "Commissioner,Ministry,Zone\n";
    let line1 = format!("{},{},{}\n",commissioner1[0],commissioner1[1],commissioner1[2]);
    let line2 = format!("{},{},{}\n",commissioner2[0],commissioner2[1],commissioner2[2]);
    let line3 = format!("{},{},{}\n",commissioner3[0],commissioner3[1],commissioner3[2]);
    let line4 = format!("{},{},{}\n",commissioner4[0],commissioner4[1],commissioner4[2]);
    let line5 = format!("{},{},{}\n",commissioner5[0],commissioner5[1],commissioner5[2]);

    let mut file = std::fs::File::create("CONVICTED_MINISTERS.csv").expect("create CONVICTED_MINISTERS.csv");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(line1.as_bytes()).expect("write failed");
    file.write_all(line2.as_bytes()).expect("write failed");
    file.write_all(line3.as_bytes()).expect("write failed");
    file.write_all(line4.as_bytes()).expect("write failed");
    file.write_all(line5.as_bytes()).expect("write failed");
    println!("Your file has been created");
    
}