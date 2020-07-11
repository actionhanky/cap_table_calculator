use std::io;

fn main() {

    //create initial cap table
    let investors: Vec<Investor> = Vec::new();

    println!("what is the company name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let company_name: String = input.trim().parse().unwrap();

    println!("how many shares are there?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let shares: i64 = input.trim().parse().unwrap();

    let mut captable = Captable::new(company_name,
        shares, investors);

    //add investors
    println!("How many investors?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut investor_counter: i32 = input.trim().parse().unwrap();
    println!("This many investors {}", investor_counter);

    while investor_counter != 0 {

        println!("What is the investor name?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let investor_name: String = input.trim().parse().unwrap();

        println!("How many shares do they hold?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let investor_shares: i64 = input.trim().parse().unwrap();

        //investors.push(Investor::new(investor_name, investor_shares));
        captable.add_investor(Investor::new(investor_name, investor_shares));

        //println!("Added: {:?}", investors);
        //println!("{}!", investor_counter);
        investor_counter -= 1;
    }

    for (i, x) in captable.investors.iter().enumerate() {
        println!("Investor number {} is {:?}", i, x.name);
    }

    println!("DONE");

}

//Captable implementation:
#[derive(Debug)]
struct Captable {
    company_name: String,
    shares: i64,
    investors: Vec<Investor>,
}

impl Captable {
    pub fn new(company_name: String, shares: i64, investors: Vec<Investor>) -> Captable {
        Captable {
            company_name: company_name,
            shares: shares,
            investors: investors,
        }
    }

    pub fn add_investor(&mut self, investor: Investor){
        self.investors.push(investor);
    }
}

//Investor implementation:
#[derive(Debug)]
struct Investor {
    name: String,
    number_of_shares: i64,
}

impl Investor {
    pub fn new(name: String, number_of_shares: i64) -> Investor {
        Investor {
            name: name,
            number_of_shares: number_of_shares,
        }
    }
}
