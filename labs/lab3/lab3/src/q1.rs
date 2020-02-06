pub fn q1() {
    println!("Question 1:");
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];
    search_member(&mut groups, "Jack");
    search_member(&mut groups, "Sue");
    search_member(&mut groups, "Brenda");
    search_member(&mut groups, "Jum");
    println!();
}

fn search_member(groups: &mut [[&str; 4]; 6], name: &str) {
    for (group_number, group) in groups.iter().enumerate(){
        if let Some(i) = group.iter().position(|&s| s == name) {
            print!("yes {} is in group {}", name, group_number);
            if i == 0 {
                print!(" and is a group leader");
            } else {
                print!(" and is not a group leader");
            }
            println!(".");
            return;
        }
    }
    println!("no {} is not in any group", name);
}
