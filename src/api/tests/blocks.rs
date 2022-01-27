use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_blocks_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.blocks.blocks_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_blocks_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.blocks.blocks_by_name().unwrap();
        assert!(by_name.get("dirt").is_some());
        assert!(by_name.get("stone").is_some());
        assert_eq!(by_name.get("grass").unwrap().stack_size, 64)
    }
}

#[test]
pub fn test_blocks_by_id() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_id = api.blocks.blocks().unwrap();
        assert!(by_id.get(&1).is_some());
        assert!(by_id.get(&5).is_some());
    }
}

#[test]
pub fn test_block_states() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.blocks.blocks_by_name().unwrap();

        let air = by_name.get("air").unwrap();
        if let Some(states) = &air.states {
            // Air has no states.
            assert_eq!(states.len(), 0);
        }

        let water = by_name.get("water").unwrap();
        if let Some(states) = &water.states {
            // Water has states.
            assert_ne!(states.len(), 0);
        }
    }
}
