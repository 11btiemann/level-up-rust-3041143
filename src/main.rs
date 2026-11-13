fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() { None 
    } else {
        let length = a.len();
        a.sort_by(f32::total_cmp);
        if length.is_multiple_of(2) {
            Some((a[length/2-1] + a[length/2])/2.0)
        } else {
            Some(a[(length-1)/2])
        }
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0, 6.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
