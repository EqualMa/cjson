use core::ops::{Add, Mul};

pub(crate) struct SizeHint(pub (usize, Option<usize>));

impl Mul<usize> for SizeHint {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let (lower, upper) = self.0;
        Self((
            lower.saturating_mul(rhs),
            match upper {
                Some(upper) => upper.checked_mul(rhs),
                None => None,
            },
        ))
    }
}

impl Add<usize> for SizeHint {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        let (lower, upper) = self.0;
        Self((
            lower.saturating_add(rhs),
            match upper {
                Some(upper) => upper.checked_add(rhs),
                None => None,
            },
        ))
    }
}

impl Add for SizeHint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a_lower, a_upper) = self.0;
        let (b_lower, b_upper) = rhs.0;

        let lower = a_lower.saturating_add(b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };

        Self((lower, upper))
    }
}
