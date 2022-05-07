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
        pub name: String,
        pub notes: String,
        pub tags: String,
    }

}
