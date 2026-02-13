use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default())
        .add_systems(Update, (spawn, update));
}

#[derive(Debug, Default, Component)]
pub struct ProgressBar {
    pub vertical: bool,
    pub color: Color,
    pub color_texture: Handle<Image>,
    pub border_color: Color,
    pub progress: f32,
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct ProgressBarMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: Vec4,
    /// Represents how much of the image is visible
    /// Goes from 0 to 1
    /// A `Vec4` is used here because Bevy with webgl2 requires that uniforms are 16-byte aligned but only the first component is read.
    #[uniform(1)]
    slider: Vec4,
    /// Image used to represent the slider
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
    /// Color of the image's border
    #[uniform(4)]
    border_color: Vec4,
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/progress_bar.wgsl".into()
    }
}

fn spawn(
    mut commands: Commands,
    query: Query<(Entity, &ProgressBar), Added<ProgressBar>>,
    mut ui_materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    for (entity, progress_bar) in query.iter() {
        commands
            .entity(entity)
            .insert(MaterialNode(ui_materials.add(ProgressBarMaterial {
                color: progress_bar.color.to_linear().to_vec4(),
                slider: Vec4::splat(0.0),
                color_texture: progress_bar.color_texture.clone(),
                border_color: progress_bar.border_color.to_linear().to_vec4(),
            })));
    }
}

// fn setup(
//     mut commands: Commands,
//     mut ui_materials: ResMut<Assets<ProgressBarMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     commands
//         .spawn((
//             Node {
//                 width: percent(100),
//                 height: percent(100),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             ZIndex(3),
//         ))
//         .with_children(|parent| {
//             let banner_scale_factor = 0.5;
//             parent.spawn((
//                 Node {
//                     position_type: PositionType::Absolute,
//                     width: px(905.0 * banner_scale_factor),
//                     height: px(363.0 * banner_scale_factor),
//                     border: UiRect::all(px(20)),
//                     border_radius: BorderRadius::all(px(20)),
//                     ..default()
//                 },
//                 MaterialNode(ui_materials.add(ProgressBarMaterial {
//                     color: LinearRgba::WHITE.to_f32_array().into(),
//                     slider: Vec4::splat(0.5),
//                     color_texture: asset_server.load("branding/banner.png"),
//                     border_color: LinearRgba::WHITE.to_f32_array().into(),
//                 })),
//             ));
//         });
// }

// Fills the slider slowly over 2 seconds and resets it
// Also updates the color of the image to a rainbow color
// fn animate(
//     mut materials: ResMut<Assets<ProgressBarMaterial>>,
//     q: Query<&MaterialNode<ProgressBarMaterial>>,
//     time: Res<Time>,
// ) {
//     let duration = 2.0;
//     for handle in &q {
//         if let Some(material) = materials.get_mut(handle) {
//             // rainbow color effect
//             let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
//             let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
//             material.color = new_color.to_linear().to_vec4();
//             material.slider.x =
//                 ((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
//             material.slider.y =
//                 ((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
//             material.border_color = border_color.to_linear().to_vec4();
//         }
//     }
// }

fn update(
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    query: Query<(&ProgressBar, &MaterialNode<ProgressBarMaterial>)>,
) {
    for (progress_bar, material) in query.iter() {
        if let Some(material) = materials.get_mut(material) {
            material.color = progress_bar.color.to_linear().to_vec4();
            if progress_bar.vertical {
                material.slider.x = progress_bar.progress;
                material.slider.y = 1.0;
            } else {
                material.slider.x = 1.0;
                material.slider.y = progress_bar.progress;
            }
            material.border_color = progress_bar.border_color.to_linear().to_vec4();
        }
    }
}
