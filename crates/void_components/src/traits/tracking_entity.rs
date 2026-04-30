use gpui::Entity;

pub trait TrackingEntity: Sized {
    fn update_value(entity: &mut Entity<Self>, key: String);
}
