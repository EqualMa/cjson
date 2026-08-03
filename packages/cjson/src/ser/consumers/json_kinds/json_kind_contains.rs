use super::{super::yes_or_no::YesOrNo, JsonKind};

pub trait JsonKindContains {
    type Contains<Other: JsonKind>: YesOrNo;

    type StringContainsSelf: YesOrNo;
    type ArrayContainsSelf: YesOrNo;
    type ObjectContainsSelf: YesOrNo;
}
