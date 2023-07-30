#[derive(Debug)]
pub struct Bicycle {
    id: i32,
    // price: f32,
    is_available: bool,
    is_active: bool,
    total_distance: f32,
    maintenance_every: f32,
}

// Create a bicycle
pub fn create_bicycle(
    id: i32,
    is_available: bool,
    is_active: bool,
    total_distance: f32,
    maintenance_every: f32
) -> Bicycle {
    Bicycle {
        id,
        is_available,
        is_active,
        total_distance,
        maintenance_every,
    }
}
