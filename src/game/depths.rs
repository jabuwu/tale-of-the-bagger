use crate::common::DepthLayer;

pub const DEPTH_BACKGROUND: DepthLayer = DepthLayer::Background(0.);
pub const DEPTH_BACKGROUND_FRONT: DepthLayer = DepthLayer::Background(0.1);
pub const DEPTH_CUSTOMER: DepthLayer = DepthLayer::Background(0.2);

pub const DEPTH_DESK: DepthLayer = DepthLayer::Foreground(0.);
pub const DEPTH_BAG: DepthLayer = DepthLayer::Foreground(0.1);
