use std::ops::Add;

use app_units::Au;
use euclid::num::Zero;
use webrender_api::units::{LayoutRect, LayoutPoint, LayoutSize};

pub mod context;
pub mod display_list;
pub mod fragment;
pub mod inline;
pub mod widget;

#[derive(Clone, Copy, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}


impl Size<Au> {
    pub fn to_layout(&self, scale_factor: f32) -> LayoutSize {
        LayoutSize::new(self.width.to_f32_px() * scale_factor, self.height.to_f32_px() * scale_factor)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Point<T> {
    pub i: T,
    pub b: T,
}

impl<T> Point<T> {
    pub fn new(i: T, b: T) -> Self {
        Self {
            i,
            b,
        }
    }
}

impl<T> Add for Point<T>
where
    T: Copy + Add<T, Output = T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            b: self.b + other.b,
        }
    }
}

impl Point<Au> {
    pub fn to_layout(&self, scale_factor: f32) -> LayoutPoint {
        LayoutPoint::new(self.i.to_f32_px() * scale_factor, self.b.to_f32_px() * scale_factor)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect<T> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T> Rect<T>
where T: Copy + Add<T, Output = T> {

    pub fn translate(&self, point: Point<T>) -> Self {
        Self {
            origin: Point {
                i: self.origin.i + point.i,
                b: self.origin.b + point.b,
            },
            size: self.size,
        }
    }
}

impl Rect<Au> {
    pub fn to_layout(&self, scale_factor: f32) -> LayoutRect {
        LayoutRect::from_origin_and_size(
            self.origin.to_layout(scale_factor),
            self.size.to_layout(scale_factor)
        )
    }
}

#[derive(Clone, Copy, Default)]
pub struct Sides<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T> Sides<T> {
    pub fn zero() -> Sides<T>
    where T: Zero
    {
        Sides {
            left: Zero::zero(),
            right: Zero::zero(),
            top: Zero::zero(),
            bottom: Zero::zero(),
        }
    }
}

pub struct Constraint {
    pub size: Size<Au>
}

impl Constraint {
    pub fn new(size: Size<Au>) -> Self {
        Self {
            size,
        }
    }
}
