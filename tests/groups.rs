use testutil::*;
use seven_client::groups::{Group, Groups};

mod testutil;

fn init_client() -> Groups {
    Groups::new(get_client())
}

#[test]
fn create() {
    let params = Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None
    };
    let client = init_client();
    let result = client.create(params);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.members_count.unwrap_or_default(), 0);
}

#[test]
fn delete() {
    let params = Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None
    };
    let client = init_client();
    let result = client.create(params);
    let group = result.unwrap();

    let result = client.delete(group.id.unwrap_or_default(), false);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.success, true);
}

#[test]
fn get() {
    let params = Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None
    };
    let client = init_client();
    let result = client.create(params);
    let group = result.unwrap();


    let result = client.get(group.id.unwrap_or_default());
    let response = result.unwrap();

    assert_ne!(response.name, "");
}

#[test]
fn all() {
    let client = init_client();
    let result = client.all();
    let response = result.unwrap();
    assert_eq!(response.paging_metadata.offset, 0);

    for group in response.data.iter() {
        assert_ne!(group.id.unwrap_or_default(), 0);
        assert_ne!(group.name, "");
    }
}
