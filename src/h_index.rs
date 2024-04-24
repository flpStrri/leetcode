// # H-Index
// Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.
//
// According to the definition of h-index on Wikipedia:
// The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.

fn solution(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    let papers_count = citations.len();
    for h_candidate in 0..papers_count {
        if (papers_count - h_candidate) as i32 <= citations[h_candidate] {
            return (papers_count - h_candidate) as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let citations = vec![3, 0, 6, 1, 5];
        assert_eq!(solution(citations), 3);
    }

    #[test]
    fn example_2() {
        let citations = vec![1, 3, 1];
        assert_eq!(solution(citations), 1);
    }

    #[test]
    fn example_3() {
        let citations = vec![0, 0, 0];
        assert_eq!(solution(citations), 0);
    }
}
