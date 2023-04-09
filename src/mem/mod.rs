/// Allows you to expose private fields of a struct
#[macro_export]
macro_rules! mem {
    // Casting
    (turn $obj:ident into $t:ty) => {
        $crate::mem!(@CAST $obj;$t)
    };
    (cast $obj:ident to $t:ty) => {
        $crate::mem!(@CAST $obj;$t)
    };
    ($obj:ident as $t:ty) => {
        $crate::mem!(@CAST $obj;$t)
    };
    (@CAST $obj:ident;$t:ty) => {
        $crate::mem_internals!(@CAST $obj;$t)
    };

    // Transforrming
    ($obj:ident as {
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $crate::mem!(@REVEAL $obj;$obj {
            $($field_vis $field_name: $field_type,)*
        })
    };
    (turn $obj:ident into {
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $crate::mem!(@REVEAL $obj;$obj {
            $($field_vis $field_name: $field_type,)*
        })
    };
    (turn $obj:ident into {
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    } named $name:ident) => {
        $crate::mem!(@REVEAL $obj;$name {
            $($field_vis $field_name: $field_type,)*
        })
    };
    (@REVEAL $obj:ident;$name:ident{
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $crate::mem_internals!(@FROM $obj;$name {
            $($field_vis $field_name: $field_type,)*
        })
    };


    () => ();
}

/// Internals of the mem! macro
#[macro_export(local_inner_macros)]
macro_rules! mem_internals {
    (@FROM $obj:expr;$name:ident {
        $($field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        {
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            struct $name {
                $($field_vis $field_name: $field_type,)*
            }
            $crate::mem_internals!(@CAST $obj;$name)
        }
    };
    (@CAST $obj:expr;$t:ty) => {
        {
            let res: $t = core::mem::transmute($obj);
            res
        }
    };

    () => ();
}
