fn main(){
    //print first 4 characters
    let sentences = "My name is Hanif".to_string();
    println!("{}",&sentences[0..=4]);
    //concatenate the string
    let description = format!("Title: Great Gosby\n{}",sentences);
    println!("{}",description);
    //iterate over the characters
    for c in sentences.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel : {}",c),
            _ => continue,
        }
    }
    // split words
    let words : Vec<&str> = sentences.split_whitespace().collect();
    println!("{:?}",words);
    // reverrse the string
    let reversed = sentences.chars().rev().collect::<String>();
    println!("{}",reversed);
}