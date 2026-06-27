// Codewars — Reverse String (kata 5168bb5dfe9a00b126000018)
// https://www.codewars.com/kata/5168bb5dfe9a00b126000018

fn main() {
    let resultado = solution("world");
    println!("{resultado}");
}

/// Complete the solution so that it reverses the string passed into it.
///
/// 'world'  =>  'dlrow'
/// 'word'   =>  'drow'
fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }
}
