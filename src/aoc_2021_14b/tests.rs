#[test]
fn it_has_the_given_difference_between_the_quantity_of_the_most_and_least_elements_after_40_steps()
{
    let output = super::determine_polymer_pairs_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        40,
    );
    assert_eq!(
        2_188_189_693_529,
        super::greatest_minus_least_element_qty(&output)
    );
}

fn starting_polymer_for_tests() -> [char; 4] {
    ['N', 'N', 'C', 'B']
}

fn rules_for_tests() -> [((char, char), char); 16] {
    [
        (('C', 'H'), 'B'),
        (('H', 'H'), 'N'),
        (('C', 'B'), 'H'),
        (('N', 'H'), 'C'),
        (('H', 'B'), 'C'),
        (('H', 'C'), 'B'),
        (('H', 'N'), 'C'),
        (('N', 'N'), 'C'),
        (('B', 'H'), 'H'),
        (('N', 'C'), 'B'),
        (('N', 'B'), 'B'),
        (('B', 'N'), 'B'),
        (('B', 'B'), 'N'),
        (('B', 'C'), 'B'),
        (('C', 'C'), 'N'),
        (('C', 'N'), 'C'),
    ]
}
