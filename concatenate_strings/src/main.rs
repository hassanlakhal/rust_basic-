fn concatenate_strings(s1 : String, s2 : String) -> String{

    let mut concat_string = String::from("");
    concat_string.push_str(&s1);
    concat_string.push_str(&s2);
    concat_string

}

fn main() {

    let s1 = String::from("Hello ");
    let s2 = String::from("World!!");
    let a = concatenate_strings(s1,s2);
    println!("{}",a);
}
