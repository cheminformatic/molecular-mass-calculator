use std::collections::HashMap;
use std::io;

fn get_atomic_masses() -> HashMap<&'static str, f64> {
    let mut masses = HashMap::new();

    masses.insert("H", 1.008);
    masses.insert("He", 4.0026);
    masses.insert("Li", 6.94);
    masses.insert("Be", 9.0122);
    masses.insert("B", 10.81);
    masses.insert("C", 12.011);
    masses.insert("N", 14.007);
    masses.insert("O", 15.999);
    masses.insert("F", 18.998);
    masses.insert("Ne", 20.180);
    masses.insert("Na", 22.990);
    masses.insert("Mg", 24.305);
    masses.insert("Al", 26.982);
    masses.insert("Si", 28.085);
    masses.insert("P", 30.974);
    masses.insert("S", 32.06);
    masses.insert("Cl", 35.45);
    masses.insert("Ar", 39.948);
    masses.insert("K", 39.098);
    masses.insert("Ca", 40.078);
    masses.insert("Sc", 44.956);
    masses.insert("Ti", 47.867);
    masses.insert("V", 50.942);
    masses.insert("Cr", 51.996);
    masses.insert("Mn", 54.938);
    masses.insert("Fe", 55.845);
    masses.insert("Co", 58.933);
    masses.insert("Ni", 58.693);
    masses.insert("Cu", 63.546);
    masses.insert("Zn", 65.38);
    masses.insert("Ga", 69.723);
    masses.insert("Ge", 72.63);
    masses.insert("As", 74.922);
    masses.insert("Se", 78.971);
    masses.insert("Br", 79.904);
    masses.insert("Kr", 83.798);
}

fn parse_formula(formula: &str) -> Vec<(String, usize)> {
    let mut elements = Vec::new();
    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut element = String::new();

        if chars[i].is_ascii_uppercase() {
            element.push(chars[i]);
            i += 1;

            if i < chars.len() && chars[i].is_ascii_lowercase() {
                element.push(chars[i]);
                i += 1;
            }

            let mut number_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                number_str.push(chars[i]);
                i += 1;
            }

            let count = if number_str.is_empty() {
                1
            } else {
                number_str.parse::<usize>().unwrap_or(1)
            };

            elements.push((element, count));
        } else {
            i += 1;
        }
    }

    elements
}

fn calculate_molar_mass(elements: &[(String, usize)], masses: &HashMap<&str, f64>) -> f64 {
    let mut total_mass = 0.0;

    for (element, count) in elements {
        if let Some(mass) = masses.get(element.as_str()) {
            total_mass += mass * (*count as f64);
        } else {
            println!("⚠️ Élément inconnu : {}", element);
        }
    }

    total_mass
}

fn main() {
    println!("Entre la formule brute (ex: C6H12O6, H2SO4, NaCl):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let masses = get_atomic_masses();
    let elements = parse_formula(input);

    println!("Éléments détectés et quantités : {:?}", elements);

    let molar_mass = calculate_molar_mass(&elements, &masses);

    println!("Masse molaire totale : {:.3} g/mol", molar_mass);
}
