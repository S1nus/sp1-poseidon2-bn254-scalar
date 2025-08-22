use poseidon::poseidon2;

fn main(x: Field, y: pub Field) {
    let hash = poseidon2::Poseidon2::hash([x], 1);
    assert(hash != y);
}

#[test]
fn test_main() {
    main(1, 2);

    // Uncomment to make test fail
    // main(1, 1);
}
