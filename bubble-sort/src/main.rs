fn main() {
    println!("Hello, world!");
    let mut list = vec![3,2,1,4,7,-1,-2,4,6,9,12,2];
    bubble_sort(&mut list); 
}

fn bubble_sort(list: &mut [i32]) {
    let mut i = 0;
    while i < list.len() - 1 {
        let mut swapped = false;
        let mut j = 0;
        while j < list.len() - i - 1 {
            let current = list[j];
            let next = list[j+1];
            if current > next {
                list[j] = next;
                list[j+1] = current;
                swapped = true
            }
            j += 1;
        }
        if !swapped {
            break;
        }
        i += 1;
    }
    
}

#[test]
fn test_bubble_sort() {
    let mut list = vec![14,3,2,1,4,7,-1,-2,4,6,9,12];
    bubble_sort(&mut list);

    println!("list {:?}", list);
    assert_eq!(list[0], -2)
}