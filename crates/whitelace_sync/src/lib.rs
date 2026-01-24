#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::iter_without_into_iter)]
#![allow(dead_code)]
#![no_std]

use core::{
    any::TypeId,
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use bevy::{
    ecs::{
        define_label,
        intern::Interned,
        query::{QueryData, QueryFilter, QueryIter},
        schedule::ScheduleLabel,
        system::{IntoObserverSystem, ScheduleSystem, SystemParam},
    },
    prelude::*,
};
use whitelace_core::{main::Subworld, map::Map};

#[derive(Component, Debug, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SyncTarget(pub Entity);

#[derive(Default, Resource)]
pub struct Worlds {
    inner: Map<Interned<dyn WorldLabel + 'static>, Subworld>,
}

impl Worlds {
    pub fn get(&self, label: impl WorldLabel) -> Option<&Subworld> {
        self.inner.get(&label.intern())
    }

    pub fn get_mut(&mut self, label: impl WorldLabel) -> Option<&mut Subworld> {
        self.inner.get_mut(&label.intern())
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MainWorld;
impl WorldLabel for MainWorld {
    #[doc = r" Clones this `"]
    #[doc = stringify!(WorldLabel)]
    #[doc = r"`."]
    fn dyn_clone(&self) -> Box<dyn WorldLabel> {
        Box::new(MainWorld)
    }
}

define_label!(WorldLabel, WORLD_LABEL_INTERNER);

pub struct WorldResMut<'worlds, R: Resource, W: WorldLabel = MainWorld> {
    _resource: core::marker::PhantomData<R>,
    label: W,
    main: &'worlds mut World,
    worlds: &'worlds mut Worlds,
}

impl<W: WorldLabel + Default, R: Resource> WorldSystemParam for WorldResMut<'_, R, W> {
    type State = ();

    fn init_state(_: &mut World) -> Self::State {}

    unsafe fn get_param((): Self::State, ctx: &SystemContext) -> Self {
        unsafe {
            Self {
                _resource: core::marker::PhantomData,
                label: W::default(),
                main: &mut *ctx.main,
                worlds: &mut *ctx.worlds,
            }
        }
    }
}

impl<R: Resource, W: WorldLabel> WorldResMut<'_, R, W> {
    /// # Panics
    ///
    /// Panics if the world is not found.
    fn get_inner(&self) -> Option<&R> {
        if TypeId::of::<W>() == TypeId::of::<MainWorld>() {
            self.main.get_resource::<R>()
        } else {
            let world = self
                .worlds
                .get(self.label.intern())
                .expect("World not found");
            world.get_resource::<R>()
        }
    }

    /// # Panics
    ///
    /// Panics if the world is not found.
    fn get_inner_mut(&mut self) -> Option<Mut<'_, R>> {
        if TypeId::of::<W>() == TypeId::of::<MainWorld>() {
            self.main.get_resource_mut::<R>()
        } else {
            let world = self
                .worlds
                .get_mut(self.label.intern())
                .expect("World not found");
            world.get_resource_mut::<R>()
        }
    }
}

impl<R: Resource, W: WorldLabel> Deref for WorldResMut<'_, R, W> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.get_inner().expect("Resource not found")
    }
}

impl<R: Resource, W: WorldLabel> DerefMut for WorldResMut<'_, R, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut m = self.get_inner_mut().expect("Resource not found");
        let mut_ref = m.as_mut();
        unsafe {
            let ptr = mut_ref as *mut R;
            &mut *ptr
        }
    }
}

pub struct WorldRes<'worlds, R: Resource, W: WorldLabel = MainWorld> {
    _resource: core::marker::PhantomData<R>,
    label: W,
    main: &'worlds mut World,
    worlds: &'worlds mut Worlds,
}

impl<R: Resource, W: WorldLabel> WorldRes<'_, R, W> {
    /// # Panics
    ///
    /// Panics if the world is not found.
    fn get_inner(&self) -> Option<&R> {
        if TypeId::of::<W>() == TypeId::of::<MainWorld>() {
            self.main.get_resource::<R>()
        } else {
            let world = self
                .worlds
                .get(self.label.intern())
                .expect("World not found");
            world.get_resource::<R>()
        }
    }
}

impl<R: Resource, W: WorldLabel> Deref for WorldRes<'_, R, W> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.get_inner().expect("Resource not found")
    }
}

impl<R, W> WorldSystemParam for WorldRes<'_, R, W>
where
    R: Resource,
    W: WorldLabel + Default,
{
    type State = ();

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param(_state: Self::State, ctx: &SystemContext) -> Self {
        unsafe {
            let main = &mut *ctx.main;
            let worlds = &mut *ctx.worlds;
            WorldRes {
                _resource: core::marker::PhantomData,
                label: W::default(),
                main,
                worlds,
            }
        }
    }
}

pub struct WorldQuery<'worlds, D, F = (), W = MainWorld>
where
    W: WorldLabel,
    D: QueryData,
    F: QueryFilter,
{
    _data: core::marker::PhantomData<D>,
    _filter: core::marker::PhantomData<F>,
    state: UnsafeCell<QueryState<D, F>>,
    label: W,
    main: &'worlds mut World,
    worlds: &'worlds mut Worlds,
}

unsafe impl<D, F, W> Send for WorldQuery<'_, D, F, W>
where
    D: QueryData,
    F: QueryFilter,
    W: WorldLabel,
{
}

unsafe impl<D, F, W> Sync for WorldQuery<'_, D, F, W>
where
    D: QueryData,
    F: QueryFilter,
    W: WorldLabel,
{
    // SAFETY:
    // 1. QueryState<D, F> должен быть Sync
    // 2. Мы гарантируем, что доступ к UnsafeCell синхронизирован внешне
    // 3. Нет внутренней мутабельности без синхронизации между потоками
}

impl<D, F, W> WorldQuery<'_, D, F, W>
where
    W: WorldLabel,
    D: QueryData,
    F: QueryFilter,
{
    pub fn iter(&self) -> QueryIter<'_, '_, <D as QueryData>::ReadOnly, F> {
        let world = unsafe {
            let world = self.get_world();
            let ptr = world as *const World;
            &*ptr
        };

        let state = self.get_state_mut();
        state.iter(world)
    }

    pub fn iter_mut(&mut self) -> QueryIter<'_, '_, D, F> {
        let world = unsafe {
            let world = self.get_world_mut();
            let ptr = world as *mut World;
            &mut *ptr
        };

        let state = self.get_state_mut();
        state.iter_mut(world)
    }

    pub fn get(
        &self,
        entity: Entity,
    ) -> core::result::Result<
        <<D as QueryData>::ReadOnly as QueryData>::Item<'_, '_>,
        bevy::ecs::query::QueryEntityError,
    > {
        let world = unsafe {
            let world = self.get_world();
            let ptr = world as *const World;
            &*ptr
        };
        let state = self.get_state_mut();
        state.get(world, entity)
    }

    pub fn get_mut(
        &mut self,
        entity: Entity,
    ) -> core::result::Result<<D as QueryData>::Item<'_, '_>, bevy::ecs::query::QueryEntityError>
    {
        let world = unsafe {
            let world = self.get_world_mut();
            let ptr = world as *mut World;
            &mut *ptr
        };

        let state = self.get_state_mut();
        state.get_mut(world, entity)
    }

    fn get_world_mut(&mut self) -> &mut World {
        if TypeId::of::<W>() == TypeId::of::<MainWorld>() {
            self.main
        } else {
            self.worlds
                .get_mut(self.label.intern())
                .expect("World not found")
        }
    }

    fn get_world(&self) -> &World {
        if TypeId::of::<W>() == TypeId::of::<MainWorld>() {
            self.main
        } else {
            self.worlds
                .get(self.label.intern())
                .expect("World not found")
        }
    }

    fn get_state(&self) -> &QueryState<D, F> {
        unsafe { &*self.state.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn get_state_mut(&self) -> &mut QueryState<D, F> {
        unsafe { &mut *self.state.get() }
    }
}

pub struct SystemContext {
    main: *mut World,
    worlds: *mut Worlds,
}

pub trait WorldSystemParam {
    type State;
    fn init_state(world: &mut World) -> Self::State;
    unsafe fn get_param(state: Self::State, ctx: &SystemContext) -> Self;
}

impl<D, F, W> WorldSystemParam for WorldQuery<'static, D, F, W>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
    W: WorldLabel + Default + 'static,
{
    type State = QueryState<D, F>;

    fn init_state(world: &mut World) -> Self::State {
        world.query_filtered::<D, F>()
    }

    unsafe fn get_param(state: Self::State, ctx: &SystemContext) -> Self {
        unsafe {
            let main = &mut *ctx.main;
            let worlds = &mut *ctx.worlds;
            WorldQuery {
                _data: core::marker::PhantomData,
                _filter: core::marker::PhantomData,
                state: UnsafeCell::new(state),
                label: W::default(),
                main,
                worlds,
            }
        }
    }
}

pub struct FunctionSystem<Marker, F> {
    func: F,
    _marker: core::marker::PhantomData<Marker>,
}

macro_rules! impl_into_world_system {
    ($($P:ident),*) => {
        impl<F, $($P),*> WorldSyncSystem for FunctionSystem<($($P,)*), F>
        where
            $($P: WorldSystemParam + Send + Sync + 'static ,)*
            F: FnMut($($P),*) + Send + Sync + 'static,
        {
            fn run(&mut self, main: &mut World, worlds: &mut Worlds) {
                unsafe {
                    let ctx = SystemContext {
                        main,
                        worlds,
                    };

                    (self.func)($($P::get_param($P::init_state(main), &ctx)),*);
                }
            }
        }

        impl<F, $($P),*> IntoWorldSyncSystem<($($P,)*)> for F
        where
            $($P: WorldSystemParam + Send + Sync + 'static ,)*
            F: FnMut($($P),*) + Send + Sync + 'static,
        {
            type System = FunctionSystem<($($P,)*), F>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    func: self,
                    _marker: core::marker::PhantomData::<($($P,)*)>,
                }
            }
        }
    };
}

impl_into_world_system!(P0);
impl_into_world_system!(P0, P1);
impl_into_world_system!(P0, P1, P2);
impl_into_world_system!(P0, P1, P2, P3);
impl_into_world_system!(P0, P1, P2, P3, P4);
impl_into_world_system!(P0, P1, P2, P3, P4, P5);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_into_world_system!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_into_world_system!(
    P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14
);
impl_into_world_system!(
    P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15
);
impl_into_world_system!(
    P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16
);

pub trait WorldSyncSystem: Send + Sync + 'static {
    fn run(&mut self, main: &mut World, worlds: &mut Worlds);
}

pub trait IntoWorldSyncSystem<M> {
    type System: WorldSyncSystem;
    fn into_system(self) -> Self::System;
}

#[derive(Resource, Default)]
pub struct SyncSystems {
    inner: Vec<Box<dyn WorldSyncSystem>>,
}

impl SyncSystems {
    pub fn add_sync_system<M>(&mut self, system: impl IntoWorldSyncSystem<M>) {
        self.inner.push(Box::new(system.into_system()));
    }

    /// # Panics
    ///
    /// Panics if the `Worlds` resource is not found.
    pub fn run(&mut self, main: &mut World) {
        let mut worlds = main
            .remove_resource::<Worlds>()
            .expect("Worlds resource not found");

        for system in &mut self.inner {
            system.run(main, &mut worlds);
        }
        main.insert_resource(worlds);
    }
}

/// # Panics
///
/// Panics if the `SyncSystems` resource is not found.
pub fn sync_worlds(world: &mut World) {
    let mut systems = world
        .remove_resource::<SyncSystems>()
        .expect("SyncSystems resource not found");
    systems.run(world);
    world.insert_resource(systems);
}

pub trait MultiworldApp {
    fn add_sync_system<M>(&mut self, system: impl IntoWorldSyncSystem<M>) -> &mut Self;

    fn add_world_systems<W: WorldLabel, M>(
        &mut self,
        label: W,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;

    fn add_world_observer<W: WorldLabel, E: Event, B: Bundle, M>(
        &mut self,
        label: W,
        observer: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

impl MultiworldApp for App {
    fn add_sync_system<M>(&mut self, system: impl IntoWorldSyncSystem<M>) -> &mut Self {
        let mut resource = self.world_mut().resource_mut::<SyncSystems>();
        resource.add_sync_system(system);
        self
    }

    fn add_world_systems<W: WorldLabel, M>(
        &mut self,
        label: W,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        if label.type_id() == TypeId::of::<MainWorld>() {
            self.add_systems(schedule, systems);
        } else {
            let mut resource = self.world_mut().resource_mut::<Worlds>();
            let world = resource.get_mut(label).unwrap();
            world.add_systems(schedule, systems);
        }
        self
    }

    fn add_world_observer<W: WorldLabel, E: Event, B: Bundle, M>(
        &mut self,
        label: W,
        observer: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        if label.type_id() == TypeId::of::<MainWorld>() {
            self.add_observer(observer);
        } else {
            let mut resource = self.world_mut().resource_mut::<Worlds>();
            let world = resource.get_mut(label).unwrap();
            world.add_observer(observer);
        }
        self
    }
}

#[derive(SystemParam)]
pub struct MultiworldCommands<'w, 's> {
    pub commands: Commands<'w, 's>,
    worlds: ResMut<'w, Worlds>,
}

impl<'w> MultiworldCommands<'w, '_> {
    pub fn spawn_at(
        &'w mut self,
        label: impl WorldLabel,
        target: impl Bundle,
        visual: impl Bundle,
    ) -> (Entity, EntityCommands<'w>) {
        let world = self.worlds.get_mut(label).unwrap();
        let mut other_commands = world.commands();
        let other = other_commands.spawn(target).id();
        let mut visual = self.commands.spawn(visual);
        visual.insert(SyncTarget(other));
        (other, visual)
    }

    pub fn spawn_empty_at(&mut self, label: impl WorldLabel, target: impl Bundle) -> Entity {
        let world = self.worlds.get_mut(label).unwrap();
        let mut other_commands = world.commands();
        other_commands.spawn(target).id()
    }

    pub fn world_commands(&mut self, label: impl WorldLabel) -> Commands<'_, '_> {
        let world = self.worlds.get_mut(label).unwrap();
        world.commands()
    }
}

pub struct SyncPlugin;
impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_worlds);
        app.init_resource::<SyncSystems>();
        app.init_resource::<Worlds>();
    }
}
