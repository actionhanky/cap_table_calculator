use std::io;

fn main() {

    //create initial cap table
    let mut captable = Captable::init_captable();

    //add the investors
    captable.add_investors();

    for (i, x) in captable.investors.iter().enumerate() {
        println!("Investor number {} is {:?} and they own {} of the company", 
        i, x.name, format!("{:.1}%", 100.0 * x.ownership(&captable)));
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

    pub fn init_captable() -> Captable {

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
        Captable::new(company_name, shares, investors)
    }

    pub fn add_investors(&mut self){
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
            self.add_investor(Investor::new(investor_name, investor_shares));

            //println!("Added: {:?}", investors);
            //println!("{}!", investor_counter);
            investor_counter -= 1;
        }
    }

    fn add_investor(&mut self, investor: Investor){
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

    pub fn ownership(&self,  captable: &Captable) -> f64 {
        self.number_of_shares as f64/captable.shares as f64
    }
}
