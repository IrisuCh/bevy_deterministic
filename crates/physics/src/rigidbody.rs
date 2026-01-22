use bevy::prelude::*;
use bevy_deterministic::math::{FVec3, Fx, IntoFx, fx};
use bridge::{Time, transform::FixedTransform};

use crate::collision::Collider;

//TODO firction

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BodyType {
    #[default]
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(Collider)]
pub struct Rigidbody {
    pub body: BodyType,
    pub velocity: FVec3,
    pub freeze: bool,
    pub mass: Fx,

    // Трение и сопротивление
    pub linear_damping: Fx,  // Воздушное сопротивление (0-1)
    pub angular_damping: Fx, // Сопротивление вращению
    pub friction: Fx,        // Коэффициент трения о поверхности (0-1)
    pub restitution: Fx,     // Упругость (0-1)

    // Вращение (если нужно)
    //pub angular_velocity: FVec3,

    // Внутренние состояния
    total_force: FVec3,
    total_torque: FVec3,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            body: BodyType::Dynamic,
            velocity: FVec3::ZERO,
            freeze: false,
            mass: fx!(1.0),
            linear_damping: fx!(0.01), // Немного сопротивления по умолчанию
            angular_damping: fx!(0.05),
            friction: fx!(0.2),
            restitution: fx!(0.5),
            //angular_velocity: FVec3::ZERO,
            total_force: FVec3::ZERO,
            total_torque: FVec3::ZERO,
        }
    }
}

impl Rigidbody {
    #[must_use]
    pub fn dynamic(mass: impl IntoFx) -> Self {
        Self {
            body: BodyType::Dynamic,
            mass: mass.into_fx(),
            ..default()
        }
    }

    #[must_use]
    pub fn kinematic() -> Self {
        Self {
            body: BodyType::Kinematic,
            ..default()
        }
    }

    pub fn set_body(&mut self, body_type: BodyType) {
        self.body = body_type;
    }

    pub fn set_velocity_xz3(&mut self, vec: FVec3) {
        self.velocity.x = vec.x;
        self.velocity.z = vec.z;
    }

    pub fn set_velocity_xz(&mut self, x: impl IntoFx, z: impl IntoFx) {
        self.velocity.x = x.into_fx();
        self.velocity.z = z.into_fx();
    }

    pub fn add_force(&mut self, force: FVec3) {
        self.total_force += force;
    }

    pub fn add_force_at_point(&mut self, force: FVec3, point: FVec3, center_of_mass: FVec3) {
        self.total_force += force;
        self.total_torque += (point - center_of_mass).cross(force);
    }

    pub fn add_impulse(&mut self, impulse: FVec3) {
        // Импульс напрямую меняет скорость: Δv = impulse / m
        self.velocity += impulse / self.mass;
    }

    #[must_use]
    pub const fn is_kinematic(&self) -> bool {
        matches!(self.body, BodyType::Kinematic)
    }

    #[must_use]
    pub const fn is_dynamic(&self) -> bool {
        matches!(self.body, BodyType::Dynamic)
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        matches!(self.body, BodyType::Static)
    }
}

pub(crate) fn apply_velocity(
    time: Res<Time>,
    mut entities: Query<(&mut FixedTransform, &mut Rigidbody)>,
) {
    let delta_time = time.delta_time();
    let gravity = FVec3::new(Fx::ZERO, fx!(-9.81), Fx::ZERO);

    for (mut transform, mut rigidbody) in &mut entities {
        if rigidbody.freeze {
            continue;
        }

        match rigidbody.body {
            BodyType::Static => continue,
            BodyType::Dynamic => {
                // 1. Применяем гравитацию (сила, зависящая от массы)
                let gravitational_force = gravity * rigidbody.mass;
                rigidbody.add_force(gravitational_force);

                // 2. Применяем воздушное сопротивление (линейный демпфинг)
                // F_damping = -damping * velocity
                let damping_force = -rigidbody.velocity * rigidbody.linear_damping;
                rigidbody.add_force(damping_force);

                // 3. Вычисляем ускорение: a = F_total / m
                let acceleration = rigidbody.total_force / rigidbody.mass;

                // 4. Интегрируем скорость: v_new = v_old + a * dt
                rigidbody.velocity += acceleration * delta_time;

                // 5. Дополнительное демпфирование для стабильности (необязательно)
                // rigidbody.velocity *= fx!(1.0) - rigidbody.linear_damping * delta_time;

                // 6. Интегрируем вращение (если нужно)
                //if rigidbody.angular_velocity != FVec3::ZERO {
                //    let angular_acceleration = rigidbody.total_torque / rigidbody.mass; // Упрощенно
                //    rigidbody.angular_velocity += angular_acceleration * delta_time;
                //    rigidbody.angular_velocity *= fx!(1.0) - rigidbody.angular_damping * delta_time;

                //    // Поворачиваем объект (упрощенно, нужен кватернион для 3D)
                //    // let rotation_axis = rigidbody.angular_velocity.normalize();
                //    // let rotation_angle = rigidbody.angular_velocity.length() * delta_time;
                //    // transform.rotate(rotation_axis, rotation_angle);
                //}
            }
            BodyType::Kinematic => {
                // Kinematic тело просто движется со своей скоростью
                // Трение и гравитация не применяются
            }
        }

        // 7. Обновляем позицию (для всех типов, кроме Static)
        transform.position += rigidbody.velocity * delta_time;

        // 8. Сбрасываем силы ДЛЯ СЛЕДУЮЩЕГО КАДРА
        rigidbody.total_force = FVec3::ZERO;
        rigidbody.total_torque = FVec3::ZERO;
    }
}

pub(crate) fn apply_material_friction(
    mut rigidbodies: Query<(&mut Rigidbody, &Collider)>,
    colliders: Query<(&Collider, &FixedTransform)>,
    time: Res<Time>,
) {
    for (mut rigidbody, collider) in &mut rigidbodies {
        if rigidbody.body != BodyType::Dynamic
            || rigidbody.freeze
            //|| collider
            //    .material
            //    .flags
            //    .contains(ColliderMaterialFlags::IGNORE_FRICTION)
            || collider.trigger
        {
            continue;
        }

        // Суммируем трение от всех контактов
        let mut total_friction_force = FVec3::ZERO;
        let mut total_adhesion_force = FVec3::ZERO;

        for contact in collider.contacts.map.values() {
            if let Ok((other_collider, _)) = colliders.get(contact.entity) {
                // Комбинированный коэффициент трения (берем среднее геометрическое)
                let combined_friction =
                    (collider.material.friction * other_collider.material.friction).sqrt();
                let combined_adhesion = collider
                    .material
                    .adhesion
                    .max(other_collider.material.adhesion);

                // Нормальная сила = масса * проекция гравитации на нормаль контакта
                let gravity = FVec3::new(Fx::ZERO, fx!(-9.81), Fx::ZERO);
                let normal_gravity = -gravity.dot(contact.contact_normal).max(Fx::ZERO);
                let normal_force = rigidbody.mass * normal_gravity;

                // Сила трения (противоположна касательной скорости)
                let tangent_velocity = contact.relative_velocity
                    - contact.contact_normal
                        * contact.relative_velocity.dot(contact.contact_normal);

                if tangent_velocity.length_squared() > fx!(0.001) {
                    let friction_force_magnitude = combined_friction * normal_force;
                    let friction_force = -tangent_velocity.normalize() * friction_force_magnitude;

                    // Ограничиваем силу трения, чтобы не создавать ускорение
                    let max_friction_force =
                        tangent_velocity.length() * rigidbody.mass / time.delta_time();
                    total_friction_force += friction_force.clamp_length_max(max_friction_force);
                }

                // Сила прилипания (если есть)
                if combined_adhesion > Fx::ZERO && contact.relative_velocity.length() < fx!(0.1) {
                    total_adhesion_force +=
                        -contact.contact_normal * combined_adhesion * normal_force;
                }
            }
        }

        // Применяем силы
        if !total_friction_force.is_near_zero() {
            rigidbody.add_force(total_friction_force);
        }

        //if !total_adhesion_force.is_near_zero() {
        //    rigidbody.add_force(total_adhesion_force);
        //}

        // Дополнительное сопротивление для скользких поверхностей
        //if collider
        //    .material
        //    .flags
        //    .contains(ColliderMaterialFlags::SLIPPERY)
        //{
        //    let slippery_damping = rigidbody.velocity * fx!(-0.1);
        //    rigidbody.add_force(slippery_damping);
        //}
    }
}
