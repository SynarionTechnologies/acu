use crate::{events::ProfileCreated, projections::ProfileView};

#[test]
fn replay_smoke() {
    let events: Vec<ProfileCreated> = Vec::new();
    let projection: Vec<ProfileView> = events
        .into_iter()
        .map(|e| ProfileView { id: e.id })
        .collect();
    assert!(projection.is_empty());
}
