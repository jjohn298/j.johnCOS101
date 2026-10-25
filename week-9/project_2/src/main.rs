use std::io::Write;

fn main() {
    let student1 = vec!["Oluchi Mardi","ACC10211111","Accounting","300"];
    let student2 = vec!["Adams Aliyu","ECO10110101","Economics","100"];
    let student3 = vec!["Shania Bolade","CSC10328828","Computer","200"];
    let student4 = vec!["Adekunle Gold","EEE11020202","Electrical","200"];
    let student5 = vec!["Blanca Edermah","MEE10202001","Mechanical","100"];

    let header = "Student name,Matric. Number,Department,Level\n";
    let line1 = format!("{},{},{},{}\n",student1[0],student1[1],student1[2],student1[3]);
    let line2 = format!("{},{},{},{}\n",student2[0],student2[1],student2[2],student2[3]);
    let line3 = format!("{},{},{},{}\n",student3[0],student3[1],student3[2],student3[3]);
    let line4 = format!("{},{},{},{}\n",student4[0],student4[1],student4[2],student4[3]);
    let line5 = format!("{},{},{},{}\n",student5[0],student5[1],student5[2],student5[3]);

    let mut file = std::fs::File::create("PAU_SIMS.csv").expect("create PAU_SIMS.csv");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(line1.as_bytes()).expect("write failed");
    file.write_all(line2.as_bytes()).expect("write failed");
    file.write_all(line3.as_bytes()).expect("write failed");
    file.write_all(line4.as_bytes()).expect("write failed");
    file.write_all(line5.as_bytes()).expect("write failed");
    println!("\nFile has been created");
    
}