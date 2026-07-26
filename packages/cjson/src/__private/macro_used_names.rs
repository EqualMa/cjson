macro_rules! define_mods {
    ({
        {
            $($mod_vis:vis $mod:ident $mod_name:tt ;)+
        }
        $use_vis:vis $use:ident __::$used:tt;
    }) => {
        $($mod_vis $mod $mod_name {
            $use_vis $use crate::ser::define_traits::$mod_name::$used;
        })+
    };
}

define_mods!({
    {
        pub mod base;
        pub mod try_;
        pub mod async_try;
    }
    pub use __::{
        //
        CONSUME_CHAINED,
        CONSUME_IN_JSON_STRING,
        CONSUME_JSON,
        CONSUME_JSON_CHUNKS,
        CONSUME_JSON_CHUNKS_FROM_INIT,
    };
});
