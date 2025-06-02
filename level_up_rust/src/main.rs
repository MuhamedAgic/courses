use std::collections::HashSet;
use std::ffi::CString;
use std::fmt::Debug;
use std::path::Path;
use chrono::{DateTime, Local};
// from the LinkedIn course: https://www.linkedin.com/learning/level-up-rust
use num::{Integer, Num};

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
    
}
