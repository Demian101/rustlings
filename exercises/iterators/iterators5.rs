// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(maps: &HashMap<String, Progress>, prog: Progress) -> usize {
    let mut count = 0;
    for val in maps.values() {
        if val == &prog {
            count += 1;
        }
    }
    count
}

fn count_iterator(maps: &HashMap<String, Progress>, prog: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    maps.iter().filter(|(_, &item)| item == prog).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], prog: Progress) -> usize {
    let mut count = 0;
    for maps in collection {
        for val in maps.values() {
            if val == &prog {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], prog: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    collection
        .iter()
        .flat_map(|map| map.iter())
        .filter(|&(_, &ele)| ele == prog)
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let maps = get_maps();
        assert_eq!(3, count_iterator(&maps, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let maps = get_maps();
        assert_eq!(1, count_iterator(&maps, Progress::Some));
    }

    #[test]
    fn count_none() {
        let maps = get_maps();
        assert_eq!(2, count_iterator(&maps, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let maps = get_maps();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&maps, progress_state),
                count_iterator(&maps, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_maps() -> HashMap<String, Progress> {
        use Progress::*;

        let mut maps = HashMap::new();
        maps.insert(String::from("variables1"), Complete);
        maps.insert(String::from("functions1"), Complete);
        maps.insert(String::from("hashmap1"), Complete);
        maps.insert(String::from("arc1"), Some);
        maps.insert(String::from("as_ref_mut"), None);
        maps.insert(String::from("from_str"), None);

        maps
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let maps = get_maps();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![maps, other]
    }
}
