fn find_smallest(vector: &Vec<i32>) -> usize {
    let mut smallest = vector[0];
    let mut smallest_index = 0;

    for a in 1..vector.len() {
        if vector[a as usize] < smallest {
            smallest = vector[a];
            smallest_index = a;
        }
    }
    smallest_index
}

fn selection_sort(mut vector: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for _i in 0..vector.len() {
        let smallest = find_smallest(&vector);
        new_vec.push(vector.remove(smallest as usize));

    }
    new_vec 

}

fn main() {
    let original_vector = vec![5,3,-1,2,2,1];
    print!("{:?}", selection_sort(original_vector));
}

// I'm not sure that they are needed here.

// #[cfg(test)]
// mod tests {
//     use::super::*;

//     #[test]
//     fn 
// }
