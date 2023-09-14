impl From<super::entities::person::Model> for crate::domain::person::Person {
    fn from(value: super::entities::person::Model) -> Self {
      crate::domain::person::Person {
        id: value.id,
        first_name: value.first_name,
        last_name: value.last_name,
      }
    }
}

impl From<crate::domain::person::Person> for super::entities::person::Model {
    fn from(value: crate::domain::person::Person) -> Self {
      Self {
        id: value.id,
        first_name: value.first_name,
        last_name: value.last_name,
      }
    }
}