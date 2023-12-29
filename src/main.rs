use std::io;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    io::stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    return your_name.trim().to_lowercase();
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    loop {
        println!("Please enter your name (leave empty to exit):");
        let name = what_is_your_name();

        if name.is_empty() {
            println!("Exiting the program. Goodbye!");
            break;
        }

        let known_visitor = visitor_list
            .iter_mut()
            .find(|visitor| visitor.name == name)
            .map(|visitor| visitor.greet_visitor());

        match known_visitor {
            Some(_) => {}
            None => {
                println!("{} is not on the visitor list.", name);
                visitor_list.push(Visitor::new(&name, "New friend"));
            }
        }
    }

    println!("Visitor List:");
    for visitor in &visitor_list {
        println!("{:?}", visitor);
    }
}
