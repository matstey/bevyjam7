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

fn update(
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    query: Query<(&ProgressBar, &MaterialNode<ProgressBarMaterial>)>,
) {
    for (progress_bar, material) in query.iter() {
        if let Some(material) = materials.get_mut(material) {
            material.color = progress_bar.color.to_linear().to_vec4();
            if progress_bar.vertical {
                material.slider.x = 1.0;
                material.slider.y = progress_bar.progress;
            } else {
                material.slider.x = progress_bar.progress;
                material.slider.y = 1.0;
            }
            material.border_color = progress_bar.border_color.to_linear().to_vec4();
        }
    }
}
