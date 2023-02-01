// a marco polo game

// if the name Marco is given the program will return Polo
// otherwise, it will return "What is your name?"
// pub exposes this fn to your main module
pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What is your name?".to_string()
    }
}
