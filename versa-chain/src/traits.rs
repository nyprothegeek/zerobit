use versa_common::traits::Config;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait Chain {
    type Config: Config;
}