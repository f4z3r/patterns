import strformat

type Car* = ref object
  model*: string
  colour*: string
  seat_count*: uint
  door_count*: uint

proc describe*(car: Car) : string =
  result = &"{car.colour} {car.model} car with {car.seat_count} seats and {car.door_count} doors"

type CarBuilder* = ref object
  car*: Car

proc new_car_builder*() : CarBuilder =
  CarBuilder(car: Car(model: "", colour: "", seat_count: 0, door_count: 0))

proc set_seat_count*(self: CarBuilder, seat_count: uint) =
  self.car.seat_count = seat_count

proc set_door_count*(self: CarBuilder, door_count: uint) =
  self.car.door_count = door_count

proc set_colour*(self: CarBuilder, colour: string) =
  self.car.colour = colour

proc set_model*(self: CarBuilder, model: string) =
  self.car.model = model

proc get_car*(self: CarBuilder) : Car =
  result = self.car


