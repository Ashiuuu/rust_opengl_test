use paste::paste;

macro_rules! offset_of {
    ( $structure: ident, $field: ident ) => {{
        let base_ptr = MaybeUninit::<$structure>::uninit().as_ptr();
        let field = ptr::addr_of!((*base_ptr).$field);
        (field as usize) - (base_ptr as usize)
    }};
}

macro_rules! concat_ident {
    ( $base: ident, $lit: literal ) => {
        [<$base $lit>]
    };
}

macro_rules! try_or_wrap {
    ( $exp: expr ) => {
        match $exp {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(err) => return std::result::Result::Err(err.into()),
        }
    };
}

pub(crate) use offset_of;
pub(crate) use try_or_wrap;
