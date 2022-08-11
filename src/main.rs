use std::io;
use std::io::Write;

mod pt;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    input.trim().to_string()
}


#[derive(Debug, Clone)]
struct Comp {
    sym: String,
    mult: f64
}


fn tokenize_formula(formula: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut sym_cap = false;

    for (i, c) in formula.chars().enumerate() {
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
            _ => return Err(format!("Invalid token '{}' encountered at position {}", c, i))
        }
    }

    // Flush the currently held value
    tokens.push(cur);
    
    return Ok(tokens);
}

fn tokens_to_units(tokens: Vec<String>) -> Result<Vec<Comp>, String> {
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
            _ => return Err("Invalid token string".to_string())
        }
    }

    units.push(cur_comp);

    Ok(units)
}

fn parse_chemical_formula(input: &str) -> Result<Vec<Comp>, String> {
    // First convert the string into just tokens that are either
    // alphabetic or numeric
    tokens_to_units(tokenize_formula(input)?)
}

fn determine_mass(units: Vec<Comp>) -> Result<f64, String> {
    let mut mass: f64 = 0.0;
    for u in units {
        let amu = pt::ELEMENTS.get(&u.sym);
        match amu {
            Some(amu) => mass += amu.mass * u.mult,
            None => return Err(format!("Element \"{}\" does not exist", u.sym))
        }
    }

    Ok(mass)
}


fn main() {
    let input = get_user_input("Enter element: ");

    let args = input.split_whitespace();

    for form in args {
        let mass = parse_chemical_formula(form).and_then(determine_mass);

        match mass {
            Err(mass) => println!("{} -> ERR: {}", form, mass),
            Ok(mass) => println!("{} -> mass = {}", form, mass)
        }
    }
}
