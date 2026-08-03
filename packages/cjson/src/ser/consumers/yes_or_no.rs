pub(crate) type No = core::convert::Infallible;

pub trait YesOrNo {}

impl YesOrNo for () {}

impl YesOrNo for No {}
