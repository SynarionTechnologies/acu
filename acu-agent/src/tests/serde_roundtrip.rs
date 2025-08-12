use chrono::Utc;
use uuid::Uuid;

use crate::events::{
    interaction::UserMessageReceived, profile::ProfileCreated, AcuEvent, EventMeta,
};

#[test]
fn serde_roundtrip() {
    let meta = EventMeta::new(Uuid::new_v4(), Uuid::new_v4(), Utc::now(), None);
    let events = vec![
        AcuEvent::ProfileCreated(ProfileCreated {
            meta: meta.clone(),
            name: "Alice".into(),
        }),
        AcuEvent::UserMessageReceived(UserMessageReceived {
            meta: meta.clone(),
            content: "hi".into(),
        }),
    ];

    for evt in events {
        let json = serde_json::to_string(&evt).expect("serialize");
        let de: AcuEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(evt, de);
    }
}
