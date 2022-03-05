#[test]
fn it_forms_the_given_polymer_after_1_step() {
    let output = super::determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        1,
    );
    assert_eq!(vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'], output);
}

#[test]
fn it_forms_the_given_polymer_after_2_steps() {
    let output = super::determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        2,
    );
    assert_eq!(
        vec!['N', 'B', 'C', 'C', 'N', 'B', 'B', 'B', 'C', 'B', 'H', 'C', 'B',],
        output
    );
}

#[test]
fn it_forms_the_given_polymer_after_3_steps() {
    let output = super::determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        3,
    );
    assert_eq!(
        vec![
            'N', 'B', 'B', 'B', 'C', 'N', 'C', 'C', 'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B', 'C',
            'H', 'B', 'H', 'H', 'B', 'C', 'H', 'B',
        ],
        output
    );
}

#[test]
fn it_forms_the_given_polymer_after_4_steps() {
    let output = super::determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        4,
    );
    assert_eq!(
        vec![
            'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B', 'C', 'C', 'N', 'B', 'C', 'N', 'C', 'C', 'N',
            'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B', 'C', 'B',
            'H', 'C', 'B', 'H', 'H', 'N', 'H', 'C', 'B', 'B', 'C', 'B', 'H', 'C', 'B',
        ],
        output
    );
}

#[test]
fn it_has_a_difference_between_the_quantity_of_the_most_and_least_elements_of_1588_after_10_steps()
{
    let output = super::determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules_for_tests(),
        starting_polymer_for_tests(),
        10,
    );
    assert_eq!(
        1588,
        super::greatest_element_qty(&output) - super::least_element_qty(&output)
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
