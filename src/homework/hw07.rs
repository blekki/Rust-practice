
fn change_case(string: String) -> String {
    let mut result: String = String::from("");
    
    // check every char inside "string" variable
    for ch in string.chars() {
        if ch.is_uppercase() {
            result += &ch.to_lowercase().to_string();
        }
        else { result += &ch.to_uppercase().to_string(); }
    }
    
    // get result
    return result;
}

fn main()
{   
    // test data
    let data = ["DATA", "Hello World!!!", "Tell me Your story", "Error 404 !$#@", "Вареники - їжа богів"];

    for i in data.iter() {
        print!("before: {:20} |", i.to_string());
        println!("after: {:20}", change_case(i.to_string()));
    }
}