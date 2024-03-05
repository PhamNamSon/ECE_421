use std::io;

fn search_member(groups: [[&str; 4]; 6], name: &str) {
    for i in 0..6{
        for j in 0..4{
            if groups[i][j].to_lowercase() == name {
                println!("{} is exist", name);
                if groups[i][0].to_lowercase() == name {
                    println!("{} is group leader of group {}", name, i+1);
                    return;
                }
                println!("{} is in group {}", name, i+1);
                return;
            } else {
                continue;
            }
        }
    }
    println!("{} is not exist", name);
    return;
}

fn main(){
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];

    loop { 
        println!("Please enter the name of the member you want to search or type 'quit' to quit");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_lowercase();
        if name == "quit"{
            break;
        }
        search_member(groups, &name);
        println!()
    }
}