use std::io;

fn main() {
    // Using macro for the vectors in the table given
    let public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_admin   = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic       = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer         = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher        = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    let mut input = String::new();
    println!("Enter the job title to check their qualification:");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let job_title = input.trim().to_lowercase();

    //Public Servant
    if job_title == "public servant" || job_title == "publicservant" {
        let mut inputnum = String::new();
        println!("How much, in years do you have experience?");
        io::stdin().read_line(&mut inputnum).expect("Not a valid string");
        let experience: u8 = inputnum.trim().parse().expect("Not a valid number");

        if experience == 0 {
            println!("Your qualified for {}", public_servant[0]);
        } else if experience >= 1 && experience <= 3 {
            println!("Your qualified for {}", public_servant[1]);
        } else if experience >= 4 && experience <= 6 {
            println!("Your qualified for {}", public_servant[2]);
        } else if experience >= 7 && experience <= 9 {
            println!("Your qualified for {}", public_servant[3]);
        } else if experience >= 10 && experience <= 11 {
            println!("Your qualified for {}", public_servant[4]);
        } else if experience >= 12 {
            println!("Your qualified for {}", public_servant[5]);
        }
    } 
    //Office Admin
    else if job_title == "office admin" || job_title == "officeadmin" {
        let mut inputnum = String::new();
        println!("How much, in years do you have experience?");
        io::stdin().read_line(&mut inputnum).expect("Not a valid string");
        let experience: u8 = inputnum.trim().parse().expect("Not a valid number");

        if experience == 0 {
            println!("Your qualified for {}", office_admin[0]);
        } else if experience >= 1 && experience <= 3 {
            println!("Your qualified for {}", office_admin[1]);
        } else if experience >= 4 && experience <= 6 {
            println!("Your qualified for {}", office_admin[2]);
        } else if experience >= 7 && experience <= 9 {
            println!("Your qualified for {}", office_admin[3]);
        } else if experience >= 10 && experience <= 11 {
            println!("Your qualified for {}", office_admin[4]);
        } else if experience >= 12 {
            println!("Your qualified for {}", office_admin[5]);
        }
    } 
    //Academic
    else if job_title == "academic" {
        let mut inputnum = String::new();
        println!("How much, in years do you have experience?");
        io::stdin().read_line(&mut inputnum).expect("Not a valid string");
        let experience: u8 = inputnum.trim().parse().expect("Not a valid number");

        if experience == 0 {
            println!("Your qualified for {}", academic[0]);
        } else if experience >= 1 && experience <= 3 {
            println!("Your qualified for {}", academic[1]);
        } else if experience >= 4 && experience <= 6 {
            println!("Your qualified for {}", academic[2]);
        } else if experience >= 7 && experience <= 9 {
            println!("Your qualified for {}", academic[3]);
        } else if experience >= 10 && experience <= 11 {
            println!("Your qualified for {}", academic[4]);
        } else if experience >= 12 {
            println!("Your qualified for {}", academic[5]);
        }
    } 
    //Lawyer
    else if job_title == "lawyer" {
        let mut inputnum = String::new();
        println!("How much, in years do you have experience?");
        io::stdin().read_line(&mut inputnum).expect("Not a valid string");
        let experience: u8 = inputnum.trim().parse().expect("Not a valid number");

        if experience == 0 {
            println!("Your qualified for {}", lawyer[0]);
        } else if experience >= 1 && experience <= 3 {
            println!("Your qualified for {}", lawyer[1]);
        } else if experience >= 4 && experience <= 6 {
            println!("Your qualified for {}", lawyer[2]);
        } else if experience >= 7 && experience <= 9 {
            println!("Your qualified for {}", lawyer[3]);
        } else if experience >= 10 && experience <= 11 {
            println!("Your qualified for {}", lawyer[4]);
        } else if experience >= 12 {
            println!("Your qualified for {}", lawyer[5]);
        }
    } 
    //Teacher
    else if job_title == "teacher" {
        let mut inputnum = String::new();
        println!("How much, in years do you have experience?");
        io::stdin().read_line(&mut inputnum).expect("Not a valid string");
        let experience: u8 = inputnum.trim().parse().expect("Not a valid number");

        if experience == 0 {
            println!("Your qualified for {}", teacher[0]);
        } else if experience >= 1 && experience <= 3 {
            println!("Your qualified for {}", teacher[1]);
        } else if experience >= 4 && experience <= 6 {
            println!("Your qualified for {}", teacher[2]);
        } else if experience >= 7 && experience <= 9 {
            println!("Your qualified for {}", teacher[3]);
        } else if experience >= 10 && experience <= 11 {
            println!("Your qualified for {}", teacher[4]);
        } else if experience >= 12 {
            println!("Your qualified for {}", teacher[5]);
        }
    } else {
        println!("Job title not recognized.");
    }
}
