use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup)
        .add_systems(Update, (
            animate_translation, animate_rotation, animate_scale
        ));
    app.run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;
    // 2D camera
    commands.spawn(Camera2d);
    // change translation
    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        AnimateTranslation,
    ));
    // change rotation
    commands.spawn((
        Text2d::new("rotation"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        AnimateRotation,
    ));
    // change scale
    commands.spawn((
        Text2d::new("scale"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
        AnimateScale,
    ));
    // text wrapping
    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_size),
            Transform::from_translation(box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in teh box\n(Unicode linebreaks"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // wrap text in the rectangle
                TextBounds::from(box_size),
                Transform::from_translation(Vec3::Z),
            ));
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.2, 0.3, 0.7), other_box_size),
            Transform::from_translation(other_box_position.extend(0.0))
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in teh box\n(AnyCharacter linebreaks)"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                // wrap text in the rectangle
                TextBounds::from(other_box_size),
                Transform::from_translation(Vec3::Z),
            ));
        });

    // font smoothing off
    commands.spawn((
        Text2d::new("this text has\nFontSmoothing::None\nand JustifyText::Center"),
        slightly_smaller_text_font
            .clone()
            .with_font_smoothing(FontSmoothing::None),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(-400.0, -250.0, 0.0)),
    ));

    for (text_anchor, color) in [
        (Anchor::TopLeft, Color::Srgba(RED)),
        (Anchor::TopRight, Color::Srgba(LIME)),
        (Anchor::BottomRight, Color::Srgba(BLUE)),
        (Anchor::BottomLeft, Color::Srgba(YELLOW)),
    ] {
        commands.spawn((
            Text2d::new(format!(" Anchor::{text_anchor:?} ")),
            slightly_smaller_text_font.clone(),
            TextColor(color),
            Transform::from_translation(250.0 * Vec3::Y),
            text_anchor,
        ));
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateScale>)>,
) {
    for mut transform in &mut query {
        let scale = (ops::sin(time.elapsed_secs()) + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
