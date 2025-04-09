use super::entities::User;

pub struct LoginCredential {
    pub user_id: uuid::Uuid,
    pub user_name: String,
    pub user_password: String,
}

impl From<User> for LoginCredential {
    fn from(user: User) -> Self {
        LoginCredential {
            user_id: user.id,
            user_name: user.name,
            user_password: user.password,
        }
    }
}
