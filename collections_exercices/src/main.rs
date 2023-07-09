fn get_median(list: &[i32]) -> f32 {
    let mut v = list.to_vec();
    let len = v.len();
    let mid = len / 2;

    v.sort();

    if len % 2 == 0 {
        (v[mid - 1] + v[mid]) as f32 / 2.0
    } else {
        v[mid] as f32
    }
}

fn main() {
    println!("* * * Exercices for collection * * *");
    {
        println!("[Exercice 1: return median when given a list of integer");
        dbg!(get_median(&[5]));
    }
}
