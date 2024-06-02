fn main() {
    let list: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 18, 19, 21, 22, 30];
    let result: u32 = binary_search(list, 5);
    println!("Result: {}", result);
}

fn binary_search(list: Vec<u32>, target: u32) -> u32 {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (high - low) / 2;
        println!("Mid: {}", mid);
        println!("List[mid]: {}", list[mid]);
        let mid_value = list[mid];
        
        if mid_value == target {
            println!("mid: {}", mid);
            return mid as u32;
        }
        
        if mid_value < target {
            high = mid - 1;
            println!("High: {}", high)
        } else {
            low = mid + 1;
            println!("Low: {}", low)
        }
    }

    return 0;
}

#[test]
fn test_binary_search() {
    let list: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 18, 19, 21, 22, 30];
    let result: u32 = binary_search(list, 19);
    assert_eq!(result, 8);
}