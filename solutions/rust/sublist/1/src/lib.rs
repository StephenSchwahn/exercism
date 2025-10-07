#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains_in_order<T: PartialEq>(longer: &[T], shorter: &[T]) -> bool {
    if shorter.len() == 0 {
        return true;
    };
    longer
        .windows(shorter.len())
        .any(|window| window == shorter)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    else if first_list.len() > second_list.len() {
        return if contains_in_order(first_list, second_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        };
    } else {
        return if contains_in_order(second_list, first_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        };
    }
}
