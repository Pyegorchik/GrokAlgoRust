use std::io::ErrorKind;

fn binary_search(vector: &Vec<i32>,item:i32) -> Result<i32,ErrorKind> {
    let mut beggining = 0;
    let mut end:i32 = <usize as TryInto<i32>>::try_into(vector.len()).unwrap() - 1;

    while beggining <= end {
        let mid:i32 = (beggining + end) / 2;
        let guess = vector[mid as usize];

        if guess == item {
            return Ok(mid);
        } else if guess > item {
            end = mid - 1;
        } else if guess < item {
            beggining = mid + 1;
        }
    }
    Err(ErrorKind::InvalidInput)
}

fn main(){
    let vector = (10..20).collect(); //array could be used.
    let item = -1;
    let result_index = binary_search(&vector,item);
    // print!("{:?}", result_index.unwrap());
    match result_index {
        Ok(index) => print!("Your index is {}", index),
        Err(error) => match error {
            ErrorKind::InvalidInput => print!("Your value {item} is out of array "),
            _ => print!("I don't know what to do!"),
        },
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_index() {
        let vector = vec![0,1,2,3,4,5,6,7,8,9,];
        let item = 1;
        let result_position = binary_search(&vector,item);
        assert_eq!(true, result_position.is_ok()); //
    }

    #[test]
    fn out_of_range() {
        let vector = vec![0,1,2,3,4,5,6,7,8,9,];
        let item = -1;
        let result_position = binary_search(&vector,item);
        assert_eq!(false, result_position.is_ok());
        
    }

}
/* Questions to handle:
1. How to test the case when item in binary_search function is not an integer? */