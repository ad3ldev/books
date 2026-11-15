pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub struct Color(pub i32, pub i32, pub i32);
pub struct Point(pub i32, pub i32, pub i32);
pub struct AlwaysEqual;
