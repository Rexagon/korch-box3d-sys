use glam::{Mat3, Quat, Vec2, Vec3, Vec3A};

use crate::{b3Matrix3, b3Quat, b3Vec2, b3Vec3};

impl From<b3Vec2> for Vec2 {
    fn from(value: b3Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vec2> for b3Vec2 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<b3Vec3> for Vec3 {
    fn from(value: b3Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vec3> for b3Vec3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<b3Vec3> for Vec3A {
    fn from(value: b3Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<Vec3A> for b3Vec3 {
    fn from(value: Vec3A) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<b3Quat> for Quat {
    fn from(value: b3Quat) -> Self {
        Self::from_xyzw(value.v.x, value.v.y, value.v.z, value.s)
    }
}

impl From<Quat> for b3Quat {
    fn from(value: Quat) -> Self {
        Self {
            v: value.xyz().into(),
            s: value.w,
        }
    }
}

impl From<b3Matrix3> for Mat3 {
    fn from(value: b3Matrix3) -> Self {
        Self::from_cols(value.cx.into(), value.cy.into(), value.cz.into())
    }
}

impl From<Mat3> for b3Matrix3 {
    fn from(value: Mat3) -> Self {
        Self {
            cx: value.x_axis.into(),
            cy: value.y_axis.into(),
            cz: value.z_axis.into(),
        }
    }
}
