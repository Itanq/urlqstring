

#[macro_export(local_inner_macros)]
macro_rules! proto_object {
    ($($obj:tt)+) => {
        object_internal!($($obj)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! object_internal {


    (@array [$($elems:expr,)*]) => {
        object_internal_vec!($($elems,)*)
    };

    (@array [$($elems:expr),*]) => {
        object_internal_vec!($($elems),*)
    };

    (@object $object:ident () () ()) => {};

    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $object.push(($($key)+, $value));
        object_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        $object.push(($($key)+, $value));
    };

    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        object_internal!(@object $object [$($key)+] (object_single_value!(true)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        object_internal!(@object $object [$($key)+] (object_single_value!(false)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr, $($rest:tt)*) $copy:tt) => {
        object_internal!(@object $object [$($key)+] (object_single_value!($value)), $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        object_internal!(@object $object [$($key)+] (object_single_value!($value)));
    };

    (@object $object:ident () (: $(rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        object_unexpected($colon);
    };

    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        object_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        object_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    ({ $($tt:tt)+ }) => {
        {
            let mut object = Vec::<(&str, &str)>::new();
            object_internal!(@object object () ($($tt)+) ($($tt)+));
            $crate::QueryParams::from( object )
        }
    };
}
   

#[macro_export]
#[doc(hidden)]
macro_rules! object_single_value {
    (true) => {
        "true"
    };

    (false) => {
        "false"
    };

    ({}) => {
        ""
    };

    ([]) => {
        ""
    };

    ([ $(tt:tt)+ ]) => {
        object_internal!(@array [] $($tt)+ )
    };

    ($expr:expr) => {
        stringify!($expr).trim_matches('"')
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! object_internal_vec {
    ($($vec:tt)*) => {
        vec![$($vec)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! object_unexpected {
    () => {};
}


#[test]
fn  test()
{
    let val = proto_object!({
        "rust": true,
        "lumin": [1,2,3,123],
        "test_string": "hello",
        "test_vec": vec!["hello","world","and","rust"]
        }
    );

    println!("value={:?}", val);
}

