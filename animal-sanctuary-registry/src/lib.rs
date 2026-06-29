use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animal = String::from(animal);

    registry
        .entry(String::from(section))
        .and_modify(|animals| {
            if !animals.contains(&animal) {
                animals.push(animal.clone());
            }
        })
        .or_insert(vec![animal]);
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut result: Vec<String> = registry
        .get(&String::from(section))
        .cloned()
        .unwrap_or(vec![]);
    result.sort();
    result
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_enimals: Vec<String> = vec![];
    for animals in registry.values() {
        for animal in animals.iter() {
            if all_enimals.contains(animal) {
                continue;
            }
            all_enimals.push(animal.clone());
        }
    }
    all_enimals.sort();
    all_enimals
}
