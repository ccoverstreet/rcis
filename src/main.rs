use std::io;
use std::io::Write;

mod pt;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    return input.trim().to_string();
}


#[derive(Debug, Clone)]
struct Comp {
    sym: String,
    mult: f64
}



fn parse_chemical_formula(input: &str) -> Vec<Comp> {
    // First convert the string into just tokens that are either
    // alphabetic or numeric

    let mut tokens: Vec<String> = Vec::new();

    let mut cur = String::new();

    let mut sym_cap = false;

    for c in input.chars() {
        match c {
            _ if c.is_uppercase()  => {
                if cur.chars().count() != 0 {
                    tokens.push(cur);
                    cur = "".to_string();
                }
                cur.push(c);
                sym_cap = true;
            },
            _ if c.is_lowercase() && sym_cap == true => {
                cur.push(c);
                sym_cap = true;
            },
            _ if c.is_digit(10) || c == '.' => {
                if sym_cap {
                    tokens.push(cur);
                    cur = "".to_string();
                }

                sym_cap = false;
                cur.push(c);
            }
            _ => todo!()
        }
    }

    tokens.push(cur);


    // Use split tokens to generate a list of formula units


    let mut cur_comp = Comp{ sym: "".to_string(), mult: 0.0 };
    let mut units: Vec<Comp> = Vec::new();

    let mut have_elem = false;
    for t in tokens.iter() {
        match t {
            _ if t.chars().all(char::is_alphabetic) => {
                if have_elem  && cur_comp.sym != "" {
                    units.push(cur_comp);
                }

                have_elem = true;
                cur_comp = Comp{ sym: t.to_string(), mult: 1.0 };
            },
            _ if t.chars().all(char::is_numeric) || t.contains(".") => {
                cur_comp.mult = t.parse::<f64>().unwrap()
            },
            _ => todo!()
        }
    }

    units.push(cur_comp);

    return units
}

fn determine_mass(units: &Vec<Comp>) {
    let mut mass: f64 = 0.0;
    for u in units {
        let amu = pt::ELEMENTS.get(&u.sym);
        match amu {
            Some(amu) => mass += amu.mass * u.mult,
            None => todo!()
        }
    }

    println!("{}", mass);
}

fn main() {
    let elems = get_user_input("Enter element: ");

    let elems = elems.split_whitespace();

    for sym in elems {
        let formula = parse_chemical_formula(sym);
        println!("{:?}", formula);
        determine_mass(&formula);
    }

    /*
    for sym in elems {
        let data = pt::ELEMENTS.get(sym);

        match data {
            Some(pt::Element{..}) => println!("{}", data.unwrap()),
            None => println!("Element \"{}\" not found", sym)
        }
    }
    */


}
