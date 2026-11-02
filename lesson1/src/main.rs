fn main() {
    // Information about me
    let name = "Prashant";
    let country = "Nepal";
    let is_student = true;
    let mut year = 1;

    println!("Name: {name}");
    println!("From: {country}");
    println!("Student: {is_student}");
    println!("Year: {year}");

    // year can change because it is "mut"
    year = year + 1;
    println!("Next year: {year}");
}