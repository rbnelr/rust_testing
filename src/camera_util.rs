use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorIcon, CursorOptions, SystemCursorIcon};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraUpdateSet;
