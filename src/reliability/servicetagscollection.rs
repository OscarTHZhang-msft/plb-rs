use std::collections::HashSet;

pub struct ServiceTagsCollection {
    tags_required_to_place: HashSet<String>,
    tags_required_to_run: HashSet<String>,
}
