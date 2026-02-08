use bevy::prelude::*;

pub fn grid_parent() -> Node {
    Node {
        display: Display::Grid,
        width: percent(100),
        height: percent(100),
        ..default()
    }
}

pub fn top_center() -> Node {
    Node {
        margin: auto().horizontal(),
        top: px(10),
        padding: UiRect::all(Val::Px(30.0)),
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..default()
    }
}

pub fn top_left() -> Node {
    Node {
        left: px(10),
        top: px(10),
        padding: UiRect::all(Val::Px(30.0)),
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..default()
    }
}

pub fn bottom_right() -> Node {
    Node {
        right: px(10),
        bottom: px(10),
        padding: UiRect::all(Val::Px(30.0)),
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..default()
    }
}

pub fn bottom_left() -> Node {
    Node {
        left: px(10),
        bottom: px(10),
        padding: UiRect::all(Val::Px(30.0)),
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..default()
    }
}
