pub mod KeePassXC {
    pub struct Entry {
        pub uuid: String,

        pub title: String,
        pub username: String,
        pub password: String,
        pub url: String,
        pub notes: String,
    }

    pub enum EntryReference {
        Unknown,
        Title,
        UserName,
        Password,
        Url,
        Notes,
        QUuid,
        CustomAttributes,
    }

    pub struct Group {
        pub uuid: String,

        pub name: String,
        pub notes: String,
        pub tags: String,

        entries: Vec<Entry>,
    }

    pub struct Database {
        entries: Vec<Entry>,
        groups: Vec<Entry>,
    }
}
