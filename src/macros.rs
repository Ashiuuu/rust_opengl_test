macro_rules! offset_of {
    ( $structure: ty, $field: ident ) => {{
        let base_ptr = MaybeUninit::<$structure>::uninit().as_ptr();
        let field = ptr::addr_of!((*base_ptr).$field);
        (field as usize) - (base_ptr as usize)
    }};
}

pub(crate) use offset_of;
