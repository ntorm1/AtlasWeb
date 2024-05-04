use std::cmp::Ordering;

pub fn is_continuous_subset<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    // Empty vector is always a subset of any vector
    if b.is_empty() {
        return true;
    }

    // Iterate over vector a to find the starting point of the subset
    for i in 0..a.len() {
        if a[i] == b[0] {
            // Found a potential starting point
            let mut j = 0;

            // Check if the subsequent elements match
            while j < b.len() && (i + j) < a.len() && a[i + j] == b[j] {
                j += 1;
            }

            // If all elements of b are matched continuously in a, return true
            if j == b.len() {
                return true;
            }
        }
    }
    // If no match is found
    false
}

pub fn sorted_union<T: Ord + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    let (mut i, mut j) = (0, 0);

    while i < vec1.len() && j < vec2.len() {
        match vec1[i].cmp(&vec2[j]) {
            Ordering::Less => {
                result.push(vec1[i].clone());
                i += 1;
            }
            Ordering::Greater => {
                result.push(vec2[j].clone());
                j += 1;
            }
            Ordering::Equal => {
                // Elements are equal, add only once
                result.push(vec1[i].clone());
                i += 1;
                j += 1;
            }
        }
    }

    // Add remaining elements from both vectors
    result.extend_from_slice(&vec1[i..]);
    result.extend_from_slice(&vec2[j..]);

    result
}
