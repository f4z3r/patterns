#!/usr/bin/env ruby -w

require 'test/unit'

require_relative 'src/factory_method.rb'
require_relative 'src/abstract_factory.rb'
require_relative 'src/builder.rb'
require_relative 'src/decorator.rb'

class TestPatterns < Test::Unit::TestCase
  def test_factory_method
    factory_1 = SedanFactory.new
    car_1 = factory_1.make_car
    # Use car, it does not matter what type of car it is.
    assert_equal(car_1.drive, "wrooom")
    assert_equal(car_1.class, Sedan)

    factory_2 = HatchbackFactory.new
    car_2 = factory_2.make_car
    assert_equal(car_2.drive, "wrooom")
    assert_equal(car_2.class, Hatchback)
  end
  def test_abstract_factory
    factory_1 = LinuxFactory.new
    button_1 = factory_1.make_button
    window_1 = factory_1.make_window
    # Use windows and buttons, it does not matter what kind of buttons/windows they are.
    assert_equal(button_1.click, "clicked")
    assert_equal(window_1.get_height, 200)
    assert_equal(button_1.class, LinuxButton)
    assert_equal(window_1.class, LinuxWindow)

    factory_2 = OSXFactory.new
    button_2 = factory_2.make_button
    window_2 = factory_2.make_window
    assert_equal(button_2.click, "clocked")
    assert_equal(window_2.get_height, 100)
    assert_equal(button_2.class, OSXButton)
    assert_equal(window_2.class, OSXWindow)
  end
  def test_builder
    builder = CarBuilder.new
    builder.set_doors(4)
    builder.set_seats(5)
    builder.set_wheels(3)
    car = builder.build
    assert_equal(car.wheels, 3)
    assert_equal(car.doors, 4)
    assert_equal(car.seats, 5)
  end
  def test_decorator
    circle = Circle.new
    red_circle = ColouredShape.new(circle, "red")
    assert_equal(red_circle.description, "red circle")
  end
end
