//-------------------------------------------------------------------------------------------------
// Macros
//-------------------------------------------------------------------------------------------------

// TODO(appcypher): Implement output part of the macro.
/// This macro is used to create a prompt.
#[macro_export]
macro_rules! prompt {
    ($str:literal) => {
        $crate::Prompt::new($str)
    };
    ([ $( $str:literal ),+ ]) => {{
        let message = concat!( $( $str ),+ );
        $crate::Prompt::new(message)
    }};
    ($( $type:ident: $others:tt ),+) => {{
        let mut prompt = $crate::PromptList::default();
        $( prompt!(@chat prompt, $type: $others ); )+
        prompt
    }};
    (@chat $prompt:ident, system: [ $( $system:tt ),+ ]) => {
        $prompt.add_message(
            concat!( $( $system ),+ ),
            vec![ $crate::Tag::Role($crate::Role::System)
        ]);
    };
    (@chat $prompt:ident, system: $system:literal ) => {
        $prompt.add_message($system, vec![ $crate::Tag::Role($crate::Role::System) ]);
    };
    (@chat $prompt:ident, user: [ $( $user:tt ),+ ]) => {
        $prompt.add_message(
            concat!( $( $user ),+ ),
            vec![ $crate::Tag::Role($crate::Role::User)
        ]);
    };
    (@chat $prompt:ident, user: $user:literal ) => {
        $prompt.add_message($user, vec![ $crate::Tag::Role($crate::Role::User) ]);
    };
    (@chat $prompt:ident, assistant: [ $( $assistant:tt ),+ ]) => {
        $prompt.add_message(
            concat!( $( $assistant ),+ ),
            vec![ $crate::Tag::Role($crate::Role::Assistant)
        ]);
    };
    (@chat $prompt:ident, assistant: $assistant:literal ) => {
        $prompt.add_message($assistant, vec![ $crate::Tag::Role($crate::Role::Assistant) ]);
    };
}

/// This macro is used to create a hashmap of key-value pairs.
#[macro_export]
macro_rules! map {
    () => {
        ::std::collections::HashMap::new()
    };
    ($( $key:expr => $value:expr),+) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

// TODO(nyprothegeek): Implement.
/// This macro is used to create a prompt map that allows selection of a prompt based on the model.
#[macro_export]
macro_rules! select {
    () => {};
}
