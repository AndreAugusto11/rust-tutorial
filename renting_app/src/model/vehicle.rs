pub trait MotorVehicle {
    fn get_id(&self) -> i32;
    fn get_is_available(&self) -> bool;
    fn get_is_active(&self) -> bool;
    fn get_maintenance_every(&self) -> i32;
    fn get_total_distance(&self) -> i32;
    fn get_consumption(&self) -> f32;
    fn get_tank_capacity(&self) -> f32;
    fn set_is_available(&mut self, is_available: bool);
    fn set_is_active(&mut self, is_active: bool);
    fn set_maintenance_every(&mut self, maintenance_every: i32);
    fn set_total_distance(&mut self, total_distance: i32);
    fn set_consumption(&mut self, consumption: f32);
    fn set_tank_capacity(&mut self, tank_capacity: f32);
    fn to_string(&self) -> String;
}
