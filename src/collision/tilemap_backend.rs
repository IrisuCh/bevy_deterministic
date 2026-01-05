
/*
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::color::Color;
use bevy::color::palettes::basic::{PURPLE, RED};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{geo, multi_polygon_as_line_strings, ColliderCreated, TiledColliderSource, TiledEvent, TiledPhysicsBackend};
use bevy_ecs_tiled::prelude::geo::MultiPolygon;
use crate::determinism::collision::Collider;

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct CollisionBackend;

impl TiledPhysicsBackend for CollisionBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        source: &TiledEvent<ColliderCreated>,
        multi_polygon: &MultiPolygon<f32>
    ) -> Vec<Entity> {
        let (name, color) = match source.event.source {
            TiledColliderSource::Object => (String::from("Custom[Object]"), Color::from(PURPLE)),
            TiledColliderSource::TilesLayer => {
                (String::from("Custom[TilesLayer]"), Color::from(RED))
            }
        };

        println!("Spawning collider {name} {:?}", color);

        vec![commands
            .spawn((Name::from(name), Collider::default()))
            // In this specific case we want to draw a mesh which require access
            // to `Assets<Mesh>` and `Assets<ColorMaterial>` resources.
            // We'll wrap everything in a custom command to get access to `World`
            // so we can retrieve these resources.
            .queue(CustomColliderCommand {
                color,
                multi_polygon: multi_polygon.clone(),
            })
            .id()]
    }
}

// Custom command implementation: nothing fancy here,
// we just store the polygons and color to use for the mesh.
struct CustomColliderCommand {
    multi_polygon: geo::MultiPolygon<f32>,
    color: Color,
}

impl EntityCommand for CustomColliderCommand {
    fn apply(self, mut entity: EntityWorldMut) {
        let mut vertices = vec![];
        multi_polygon_as_line_strings(&self.multi_polygon)
            .into_iter()
            .for_each(|ls| {
                ls.lines().for_each(|l| {
                    let points = l.points();
                    vertices.push([points.0.x(), points.0.y(), 10.]);
                    vertices.push([points.1.x(), points.1.y(), 10.]);
                });
            });

        let mut mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        let mesh_handle = entity.resource_mut::<Assets<Mesh>>().add(mesh);
        let material_handle = entity
            .resource_mut::<Assets<ColorMaterial>>()
            .add(self.color);
        entity.insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
    }
}

*/