
mod model;

use crate::model::bicycle::*;
use crate::model::truck::*;
use crate::model::car::*;

// Create and return list of trucks
fn create_truck_list() -> Vec<Truck> {
    let mut list: Vec<Truck> = Vec::new();
    list.push(create_truck(1, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(2, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(3, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(4, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(5, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(6, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(7, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(8, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(9, true, true, 0, 0, 0.0, 0.0));
    list.push(create_truck(10, true, true, 0, 0, 0.0, 0.0));
    list
}

// Create and return list of cars
fn create_car_list() -> Vec<Car> {
    let mut list: Vec<Car> = Vec::new();
    list.push(create_car(1, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(2, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(3, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(4, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(5, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(6, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(7, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(8, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(9, true, true, 0, 0, 0.0, 0.0));
    list.push(create_car(10, true, true, 0, 0, 0.0, 0.0));
    list
}

fn create_bicycle_list() -> Vec<Bicycle> {
    let mut list: Vec<Bicycle> = Vec::new();
    list.push(create_bicycle(1, true, true, 0.0, 0.0));
    list.push(create_bicycle(2, true, true, 0.0, 0.0));
    list.push(create_bicycle(3, true, true, 0.0, 0.0));
    list.push(create_bicycle(4, true, true, 0.0, 0.0));
    list.push(create_bicycle(5, true, true, 0.0, 0.0));
    list.push(create_bicycle(6, true, true, 0.0, 0.0));
    list.push(create_bicycle(7, true, true, 0.0, 0.0));
    list.push(create_bicycle(8, true, true, 0.0, 0.0));
    list.push(create_bicycle(9, true, true, 0.0, 0.0));
    list.push(create_bicycle(10, true, true, 0.0, 0.0));
    list
}

fn management_system() {
    let truck_list = create_truck_list();
    let car_list = create_car_list();
    let bicycle_list = create_bicycle_list();

    // Iterate through the truck list and print each truck
    for truck in &truck_list {
        println!("{:?}", truck);
    }

    // Iterate through the car list and print each car
    for car in &car_list {
        println!("{:?}", car);
    }

    // Iterate through the bicycle list and print each bicycle
    for bicycle in &bicycle_list {
        println!("{:?}", bicycle);
    }
}

fn main() {
    management_system();
}
