use fake::Fake;
use indexmap::IndexMap;

pub mod document_keydown;
pub mod filedropper;
pub mod table;
pub struct Row {
    pub id: String,
    pub user: User,
}

#[derive(PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub properties: IndexMap<String, String>,
    pub groups: IndexMap<String, bool>,
}

pub fn create_mock_users(n_users: usize, n_groups: usize) -> Vec<User> {
    let groups = (0..n_groups)
        .map(|_| {
            fake::faker::lorem::en::Words(2..4)
                .fake::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<_>>();

    (0..n_users)
        .map(|_| User {
            id: fake::faker::number::en::NumberWithFormat("u######").fake(),
            name: fake::faker::name::de_de::Name().fake(),
            properties: IndexMap::new(),
            groups: groups
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, group)| {
                    (
                        group,
                        fake::faker::boolean::en::Boolean(
                            ((1.0 - i as f32 / n_groups as f32) * 100.0) as u8,
                        )
                        .fake(),
                    )
                })
                .collect(),
        })
        .collect()
}

pub struct PropertyColumn {
    pub id: String,
    pub property: String,
}

impl PropertyColumn {
    pub fn new(id: String, property: String) -> Self {
        Self { id, property }
    }

    pub fn access(&self, row: &Row) -> String {
        row.user
            .properties
            .get(&self.id)
            .cloned()
            .unwrap_or_default()
    }
}

pub struct GroupColumn {
    pub id: String,
    pub group: String,
}

impl GroupColumn {
    pub fn new(id: String, group: String) -> Self {
        Self { id, group }
    }
    pub fn access(&self, row: &Row) -> bool {
        row.user.groups.get(&self.id).cloned().unwrap_or_default()
    }
}
