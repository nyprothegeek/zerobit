//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait Middleware {
    fn map<T>(&self, input: T /* meta: &Meta */) -> T;
}

pub trait DynMiddleware {}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_dyn_middleware() {
        // TODO(nyprothegeek): Check that the dyn_middleware trait object can be created and necessary type converted.
    }
}
