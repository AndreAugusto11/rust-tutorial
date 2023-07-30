use crate::model::vehicle::MotorVehicle;

#[derive(Debug)]
pub struct Car {
    id: i32,
    is_available: bool,
    is_active: bool,
    total_distance: i32,
    maintenance_every: i32,
    consumption: f32,
    tank_capacity: f32,
}

// Create a car
pub fn create_car(
    id: i32,
    is_available: bool,
    is_active: bool,
    maintenance_every: i32,
    total_distance: i32,
    consumption: f32,
    tank_capacity: f32
) -> Car {
    Car {
        id,
        is_available,
        is_active,
        maintenance_every,
        total_distance,
        consumption,
        tank_capacity,
    }
}

impl MotorVehicle for Car {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_is_available(&self) -> bool {
        self.is_available
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }
    
    fn get_maintenance_every(&self) -> i32 {
        self.maintenance_every
    }

    fn get_total_distance(&self) -> i32 {
        self.total_distance
    }
    
    fn get_consumption(&self) -> f32 {
        self.consumption
    }

    fn get_tank_capacity(&self) -> f32 {
        self.tank_capacity
    }

    fn set_is_available(&mut self, is_available: bool) {
        self.is_available = is_available;
    }

    fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    fn set_maintenance_every(&mut self, maintenance_every: i32) {
        self.maintenance_every = maintenance_every;
    }

    fn set_total_distance(&mut self, total_distance: i32) {
        self.total_distance = total_distance;
    }

    fn set_consumption(&mut self, consumption: f32) {
        self.consumption = consumption;
    }

    fn set_tank_capacity(&mut self, tank_capacity: f32) {
        self.tank_capacity = tank_capacity;
    }

    fn to_string(&self) -> String {
        format!("Car id: {}, is_available: {}, is_active: {}, maintenance_every: {}, total_distance: {}, consumption: {}, tank_capacity: {}", self.id, self.is_available, self.is_active, self.maintenance_every, self.total_distance, self.consumption, self.tank_capacity)
    }
}