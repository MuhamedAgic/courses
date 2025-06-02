use std::collections::HashSet;
use std::ffi::CString;
use std::path::Path;
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
    
    
    
    
}
