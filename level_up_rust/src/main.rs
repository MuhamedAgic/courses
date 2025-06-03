// from the LinkedIn course: https://www.linkedin.com/learning/level-up-rust

use std::ffi::CString;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::{Chars, FromStr};
use chrono::{DateTime, Local, NaiveDate, Weekday};

// 1. Calulate the median
fn my_median(elements: &Vec<f32>) -> Option<f32> {
    // T for numeric values? partial cmp doesnt work for T: num...
    if elements.len() == 0 {
        return None;
    }
    
    let mut sorted = elements.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let middle_idx: usize = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        let second_middle_idx = middle_idx - 1;
        let first = sorted[middle_idx];
        let second = sorted[second_middle_idx];
        Some((first + second) / 2.0)
    }
    else {
        let middle_idx: usize = ((sorted.len() as f32 / 2.0).floor()) as usize;
        Some(sorted[middle_idx])
    }
}

#[test]
fn median_on_empty_list() {
    assert_eq!(my_median(&vec![]), None);
}

#[test]
fn median_on_even_amount_of_elements() {
    assert_eq!(my_median(&vec![1.0, 10.0]), Some(5.5));
}

#[test]
fn median_on_uneven_amount_of_elements() {
    assert_eq!(my_median(&vec![1.0, 10.0, 100000.0]), Some(10.0));
}

#[test]
fn median_on_even_amount_of_elements_unsorted() {
    assert_eq!(my_median(&vec![1.0, 10.0, 5.0, 3.0]), Some(4.0));
}

#[test]
fn median_on_uneven_amount_of_elements_unsorted() {
    assert_eq!(my_median(&vec![1.0, 10.0, 5.0, 3.0, 10000.0]), Some(5.0));
}


fn courses_median(mut elements: Vec<f32>) -> Option<f32> {
    if elements.is_empty() {
        return None;
    }
    
    elements.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n_elements = elements.len();
    let middle = n_elements / 2;
    let elements_is_even = n_elements % 2 == 0;
    
    // more idiomatic rust
    let median = if elements_is_even {
        (elements[middle] + elements[middle - 1]) / 2.0
    } else { 
        elements[middle]
    };
    
    Some(median)
}
//===================================================================================
// 2. Find unique items
// Extra credit:
// Use generics, use T: Ord
// Retain order

fn my_unique<T: Ord + Clone>(elements: &Vec<T>) -> Vec<T> {
    let mut unique_elements: Vec<T> = Vec::new();
    for element in elements.iter() {
        if !unique_elements.contains(element) {
            unique_elements.push((*element).clone());
        }
    }
    unique_elements
}

fn courses_unique<T: Ord>(mut elements: Vec<T>) -> Vec<T> {
    elements.sort_by(|a, b| a.partial_cmp(b).unwrap());
    elements.dedup();
    elements
}
//===================================================================================
// 3. Print any text type
// Write a fn that can accept a String or a &str
// requirements:
// implement info(text: &T)
// Must accept at least String and &str
// print output

fn my_info<T: std::fmt::Display>(text: &T) {
    println!("{}", text);
}

//===================================================================================
// 4. Case insensitive sort
// sort usernames, ignore case of any letters within them
// requirements: implement sort_usernames(&mut users)
// sorted in place
// accept all unicode chars

fn my_sort_usernames(users: &mut Vec<&str>) {
    users.sort_by(|a, b| {
        a.to_lowercase().cmp(&b.to_lowercase())
    });
}


fn courses_sort_usernames<T: AsRef<str>>(users: &mut Vec<T>) {
    users.sort_by(|a, b| {
        a.as_ref()
            .to_lowercase()
            .cmp(&b.as_ref().to_lowercase())
    });
}

fn courses_sort_usernames_v2<T: AsRef<str>>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|a| { 
        a.as_ref().to_lowercase()  // cached key: once per item in list
    });
}

//===================================================================================
// 5. Convert text to morse code
// requirements: implement the morsecode trait for String
// morsecode is defined in sample code
// skip invalid input chars
// ignore case

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;
type Letter = Vec<Pulse>;
enum Pulse {
    Short,
    Long
}

fn letter_to_pulse_code(char: char) -> Option<Letter> {
    match char {
        'a' => {
            let letter = vec![
                Pulse::Long,
                Pulse::Short
            ];
            Some(letter)
        }
        'b' => {
            let letter = vec![
                Pulse::Long,
                Pulse::Short,
                Pulse::Short,
                Pulse::Short
            ];
            Some(letter)
        }
        'c' => {
            let letter = vec![
                Pulse::Long,
                Pulse::Short,
                Pulse::Long,
                Pulse::Short
            ];
            Some(letter)
        }
        _ => None
    }
}

// forgot to implement how to print stuff
impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_")
        }
    }
}

fn print_morse_code(morse_code: &Message) {
    for letter in morse_code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
}
    

// my version
impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut message = Message::new();
        for char in self.chars() {
            let morse_code = match char { 
                'a'..'z' => {
                    if let Some(letter) = letter_to_pulse_code(char) {
                        letter
                    } else {
                        Letter::new()
                    }
                },
                'A'..'Z' => {
                    let char = char.to_ascii_lowercase();
                    if let Some(letter) = letter_to_pulse_code(char) {
                        letter
                    } else {
                        Letter::new()
                    }
                }
                _ => Letter::new()
            };
            message.push(morse_code);
        }
        message
    }
}

//===================================================================================
// 6. value hand of cards
// requirements: use types given in sample code
// implement value method for the hand struct
// 1..10 face cards are 10, a = 11 or if val > 21 -> 1
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    K,
    Q,
    J,
    A
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn value(&self) -> usize {
        let mut value: usize = 0;
        let mut present_as = 0; // they call it aces_seen
        for card in self.cards.iter() {
            let val = match card { 
                Card::One => 1,
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Ten => 10,
                Card::K => 10,
                Card::Q => 10,
                Card::J => 10,
                Card::A => {
                    present_as += 1;
                    0
                },
            };
            value += val;
        }
         for i in 0..present_as {
             if value > 21 {
                 value += 1;
             } else { 
                 value += 11;
             }
         }
        value
    }
}

//===================================================================================
// 7. check deadline
// requirements
// Design importantEvent data structure
// must have name and date fields
// implement Deadline trait for ImportantEvent
// use chrono 

struct ImportantEvent {
    name: String,
    date: DateTime<Local>
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        println!("event time: {}, time now: {}", self.date, Local::now());
        Local::now() > self.date
    }
}

//===================================================================================
// 8. temperature conversion
// add to_celcius and to_farenheit

#[derive(Debug, PartialOrd, PartialEq)]
enum Scale {
    Celcius,
    Farenheit
}

#[derive(Debug)]
struct Temparature {
    degrees: f32,
    scale: Scale
}

impl Temparature {
    fn to_celcius(&mut self) {
        match self.scale {
            Scale::Celcius => (),
            Scale::Farenheit => {
                self.degrees = self.degrees * 9.0 / 5.0 + 32.0;
                self.scale = Scale::Farenheit;
            }
        }
    }

    fn to_farenheit(&mut self) {
        match self.scale {
            Scale::Farenheit => (),
            Scale::Celcius => {
                self.degrees = (self.degrees - 32.0) * 5.0 / 9.0;
                self.scale = Scale::Celcius;
            }
        }
    }
}

//===================================================================================
// 9. sum list with missing values

fn sum_with_missing(elements: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for element in elements {
        if let Some(element) = element {
            sum += element;
        }
    }
    sum
}

fn sum_with_missing_v2(elements: Vec<Option<i32>>) -> i32 {
    elements
        .iter()
        .map(|e| e.unwrap_or_default())
        .sum::<i32>()
}

//===================================================================================
// 10. check weeks between 2 dates
// requirements: weeks_between(a: &str, b: &str) -> i32
// yyyy-mm-dd
// if b < a, return negative number
// chrono crate
// handle errors

fn weeks_between(a: &str, b: &str) -> i32 {
    let date_a = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let date_b = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();
    (date_b.week(Weekday::Mon).first_day() - date_a.week(Weekday::Mon).first_day()).num_weeks() as i32
}

//===================================================================================
// 11. validate ISBN number
// requirements: std::fmt::FromStr trait for the isbn number
// validate digits
// create appropriate error type
// std fmt fromstr trait requires that you define an error type
// input too, long, too short, or failed checksum
// check digits by: mul each num by pre-assigned weights
// reduce sum to single digit, start with 10, substr the remainder of dividing the sum by 10

#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>
}

fn calculate_isbn_checksum(digits: &Vec<u8>) -> Option<u8> {
    if digits.is_empty() {
        return None;
    }
    
    let weighted_sum = digits
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            if i % 2 == 0 {
                digit * 1
            } else {
                digit * 3
            }
        })
        .sum::<u8>();

    let checkum = 10 - (weighted_sum % 10);
    Some(checkum % 10)
}
impl FromStr for Isbn {
    type Err = IsbnError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8> = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();
        
        if digits.len() > 13 {
            return Err(IsbnError::TooLong)
        } else if digits.len() < 13 {
            return Err(IsbnError::TooShort)
        }

        // remove the hyphens
        let last_digit = digits.pop();
        
        let isbn_checksum = calculate_isbn_checksum(&digits);
        if isbn_checksum.is_none() {
            return Err(IsbnError::InvalidChecksum)
        }
        
        if isbn_checksum.unwrap() > 9 
            || isbn_checksum.unwrap() < 0 
            || last_digit != isbn_checksum {
            return Err(IsbnError::InvalidChecksum)
        }
        
        Ok(Isbn {raw: s.to_string(), digits: digits})
    }
}

#[derive(Debug)]
enum IsbnError {
    TooLong,
    TooShort,
    InvalidChecksum
}

impl std::fmt::Display for IsbnError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IsbnError::TooLong => write!(f, "Isbn number too long"),
            IsbnError::TooShort => write!(f, "Isbn number too short"),
            IsbnError::InvalidChecksum => write!(f, "Invalid isbn checksum")
        }
    }
}

//===================================================================================
// 12. validate file exists
trait FileMetadata {
    fn exists(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_readable(&self) -> bool;
}

impl FileMetadata for std::path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }
    fn is_writable(&self) -> bool {
        fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    }
    fn is_readable(&self) -> bool {
        fs::File::open(self).is_ok()
    }
}

//===================================================================================
// 13. interpret rgb hex color
// parse string to rust type color
// requirements: datastructure rgb
// implement rgb channels using trait from sample code
// implement fromStr for rgb

#[derive(Debug)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Debug)]
enum ParseColorError {
    NoLeadningHash,
    InvalidHexLength,
    InvalidHexDigit
}
impl FromStr for Rgb {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("#").ok_or(ParseColorError::NoLeadningHash)?;
        
        if hex.len() != 6 {
            return Err(ParseColorError::InvalidHexLength);
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseColorError::InvalidHexDigit)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseColorError::InvalidHexDigit)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseColorError::InvalidHexDigit)?;
        Ok(Rgb{r, g, b})
    }
}


//===================================================================================
// 14. run length encoding
// requirements:
// string to encoded
// encoded to string
// str -> encode -> decode -> must be original str
// AAAAAaAA -> 5A1a2A

fn encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut encoded_s = String::from("");
    let mut current_char = s.chars().next().unwrap();
    let mut current_char_count = 0;
    
    for (i, char) in s.chars().enumerate() {
        if char == current_char {
            current_char_count += 1;   
        } else {
            encoded_s.push_str(&format!("{}{}", current_char_count.to_string(), current_char));
            current_char = char;
            current_char_count = 1;
        }
        if i == s.len() - 1{
            encoded_s.push_str(&format!("{}{}", current_char_count.to_string(), current_char));
        }
    }
    encoded_s
}

fn decode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut decoded_s = String::from("");
    let mut current_char_count = 0;

    for char in s.chars() {
        if let Some(count) = char.to_digit(10) {
            current_char_count = count;
        } else { 
            for i in 0..current_char_count {
                decoded_s.push_str(char.to_string().as_str());
            }
        }
    }
    decoded_s
}


//===================================================================================

fn main() {
    let vec_f32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 1000.0, 1001.0];
    let my_median = my_median(&vec_f32);
    let courses_median = courses_median(vec_f32);
    println!("My median f32: {:?}", my_median);
    println!("Courses median f32: {:?}", courses_median);

    let dups_vec: Vec<i32> = vec![1, 1, 1, 9, 9, 10];
    let my_unique_vec: Vec<i32> = my_unique(&dups_vec);
    let courses_unique_vec: Vec<i32> = courses_unique(dups_vec);
    println!("My unique : {:?}", my_unique_vec);
    println!("Courses unique : {:?}", courses_unique_vec);

    let owned_string = String::from("Owned hello world");
    let str = "hello world";
    let c_string = CString::new("c_string").unwrap();
    let path = Path::new("path");
    my_info(&owned_string);
    my_info(&str);
    my_info(&c_string.to_str().unwrap());
    my_info(&path.to_str().unwrap());
    
    let mut usernames = vec!["alice", "Bob", "CaRol"];
    my_sort_usernames(&mut usernames);
    println!("My sort usernames: {:?}", usernames);
    let mut usernames = vec!["alice", "Bob", "CaRol"];
    courses_sort_usernames(&mut usernames);
    println!("Courses sort usernames: {:?}", usernames);
    let mut usernames = vec!["alice", "Bob", "CaRol"];
    courses_sort_usernames_v2(&mut usernames);
    println!("Courses sort usernames v2: {:?}", usernames);
    
    let morse_string = String::from("abc");
    let morse_code = morse_string.to_morse_code();
    print!("{} in morse code is: ", morse_string);
    print_morse_code(&morse_code);
    println!();
    
    let hand = vec![Card::A, Card::Ten, Card::Three, Card::Four, Card::Five];
    let hand = Hand { cards: hand };
    println!("Hand has value: {}", hand.value());
    let hand = vec![Card::A, Card::Four, Card::Five];
    let hand = Hand { cards: hand };
    println!("Hand has value: {}", hand.value());
    
    let ie = ImportantEvent{name: "Jack".to_string(), date: Local::now()};
    println!("Is passed event? {}", ie.is_passed());
    
    let mut t = Temparature {degrees: 3.14, scale: Scale::Celcius};
    println!("Temparature is degrees Celsius: {:?}", t);
    t.to_farenheit();
    println!("Temparature is Farenheit: {:?}", t);

    let vec: Vec<Option<i32>> = vec![Some(1), Some(10), None, Some(1000)];
    let sum_missing = sum_with_missing_v2(vec);
    println!("Sum missing value: {}", sum_missing);
    let vec: Vec<Option<i32>> = vec![None, None, None, None];
    let sum_missing = sum_with_missing_v2(vec);
    println!("Sum missing value: {}", sum_missing);

    let date_a = "2000-01-01";
    let date_b = "1999-01-01";
    println!("Weeks between: {}", weeks_between(date_a, date_b));

    let isbn = Isbn::from_str("1999-01-01");
    let isbn = Isbn::from_str("1999-01-01-0023401324-0123413240");
    let isbn = Isbn::from_str("978-3-16-148410-0");
    println!("Isbn number parsed: {:?}", isbn);
    
    let path_to_file = std::path::Path::new("./src/a.txt");
    println!("Does file exist? {}" , path_to_file.exists());
    println!("Is file readable? {}" , path_to_file.is_readable());
    println!("Is file writable? {}" , path_to_file.is_writable());

    let color = String::from("#3e9841");
    // let color = String::from("#3g9841");
    // let color = String::from("3g9841");
    let rgb = Rgb::from_str(&color);
    println!("Parsed Rgb: {:?}", rgb);
    
    let str = "AAAAAaaAbCCCdd";
    let encoded_str = encode(str);
    println!("Encoded str: {}", encoded_str);
    let decoded_str = decode(&encoded_str);
    println!("Decoded str: {}", &decoded_str);
    
    
}
