#[derive(Clone, Debug)]
pub struct Ctx {
    // Authenticated user id for the current request.
    user_id: u64,
}

impl Ctx {
    // Create a request context for one user.
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
