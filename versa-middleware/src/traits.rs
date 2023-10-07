//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait Middleware {
    type Value;
    type Meta;

    /// Map the input to the output.
    fn map(&self, input: &Self::Value, meta: &Self::Meta) -> Self::Value;
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Middleware;

    mod helper {
        use crate::Middleware;

        pub struct TestMiddleware1 {}
        pub struct TestMiddleware2 {}

        impl Middleware for TestMiddleware1 {
            type Meta = ();
            type Value = String;

            fn map(&self, input: &Self::Value, _meta: &Self::Meta) -> Self::Value {
                format!("{input} - 1")
            }
        }

        impl Middleware for TestMiddleware2 {
            type Meta = ();
            type Value = String;

            fn map(&self, input: &Self::Value, _meta: &Self::Meta) -> Self::Value {
                format!("{input} - 2")
            }
        }
    }

    #[test]
    fn test_dyn_middleware() {
        let middlewares: Vec<Box<dyn Middleware<Value = String, Meta = ()>>> = vec![
            Box::new(helper::TestMiddleware1 {}),
            Box::new(helper::TestMiddleware2 {}),
        ];

        let input = String::from("Hello");
        let results = middlewares
            .iter()
            .map(|m| m.map(&input, &()))
            .collect::<Vec<_>>();

        assert_eq!(results, vec!["Hello - 1", "Hello - 2"]);
    }
}
