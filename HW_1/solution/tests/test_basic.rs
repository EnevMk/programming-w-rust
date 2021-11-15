use solution::*;

#[test]
fn test_basic() {
    let input: Vec<char> = "GC".chars().collect();
    let counter = solution::counts(&input);

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 0);
    assert_eq!(counter.t, 0);

    assert_eq!(solution::dna_complement(&input),         vec!['C', 'G']);
    assert_eq!(solution::reverse_rna_complement(&input), vec!['G', 'C']);
}

#[test]
fn test_additional() {
    let mut input: Vec<char> = "ACGTTGATAC".chars().collect();
    let count : NucleotideCounter = counts(&input);

    assert_eq!(count.a, 3);
    assert_eq!(count.c, 2);
    assert_eq!(count.g, 2);
    assert_eq!(count.t, 3);

    assert_eq!(dna_complement(&input), vec!['T', 'G', 'C', 'A', 'A', 'C', 'T', 'A', 'T', 'G']);
    input.reverse();
    assert_eq!(reverse_rna_complement(&input), vec!['U', 'G', 'C', 'A', 'A', 'C', 'U', 'A', 'U', 'G']);
}

#[test]
fn test_other_stuff() {

    let mut x : [char; 3] = ['a', 'c', 'g'];
    let x : &mut [char]= &mut x[..]; // Take a full slice of `x`.
    x[1] = 'y';
    assert_eq!(x, &['a', 'y', 'g']);

    assert_eq!(x[1], 'y');
}