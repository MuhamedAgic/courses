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



fn main() {
    let mut vec_f32: Vec<f32> = vec![1.0,2.0,3.0,4.0,1000.0,1001.0];
    let vec_f32: Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    let my_median = my_median(&vec_f32);
    let courses_median = courses_median(vec_f32);
    println!("My median f32: {:?}", my_median);
    println!("Courses median f32: {:?}", courses_median);
    
    
    
    
    
}
