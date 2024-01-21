use rust_axum_kernel::model::user::User;

pub struct SearchedUser {
    pub id: i32,
    pub name: String,
}

impl From<User> for SearchedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value,
            name: user.name,
        }
    }
}
