# This is just an example to get you started. You may wish to put all of your
# tests into a single file, or separate them into multiple `test1`, `test2`
# etc. files (better names are recommended, just make sure the name starts with
# the letter 't').
#
# To run these tests, simply execute `nimble test`.

import unittest

import patterns/builder
test "builder works":
  let builder = new_car_builder()
  builder.set_colour("red")
  builder.set_model("sedan")
  builder.set_door_count(5)
  builder.set_seat_count(5)
  var car = builder.get_car
  check car.describe == "red sedan car with 5 seats and 5 doors"
