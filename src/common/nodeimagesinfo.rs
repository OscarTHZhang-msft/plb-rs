use std::collections::HashSet;

pub struct NodeImagesInfo {
    // repo tag (registry/repository:tag) is an alias for image id
    // Put repo tags of all images together in unordered_set
    // because we want to quickly look up for specific repo tag
    repo_tags: HashSet<String>,

    // layers diff ids (diff id indentifies a single changeset)
    // Put layers of all images together in unordered_set
    // because we want to quickly look up for specific layer
    layers: HashSet<String>,

    // Total size on disk for the image ids
    total_image_size: u64,
}

impl NodeImagesInfo {
    pub fn default() -> NodeImagesInfo {
        NodeImagesInfo {
            repo_tags: HashSet::new(),
            layers: HashSet::new(),
            total_image_size: 0,
        }
    }

    pub fn new(
        repo_tags: HashSet<String>,
        layers: HashSet<String>,
        total_image_size: u64,
    ) -> NodeImagesInfo {
        NodeImagesInfo {
            repo_tags: repo_tags,
            layers: layers,
            total_image_size: total_image_size,
        }
    }
}
