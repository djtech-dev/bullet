pub type Version = Vec<u8>;
pub enum Condition {
    MajorThan(Version),
    MinorThan(Version),
    EqualThan(Version),

    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}
pub struct Dependency {
    package: String,
    condition: Condition,
}
pub struct Package {
    name: String,
    license: String,
    author: String,

    version: Version,

    dependencies: Vec<Dependency>,
}
