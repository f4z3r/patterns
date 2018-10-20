# Builder design pattern

module Patterns::Builder
  class Car
    property model : String
    property colour : String
    property seat_count : UInt8
    property door_count : UInt8

    def initialize(@model = "hatchback", @colour = "black", @seat_count = 5_u8, @door_count = 5_u8)
    end

    def description
      "#{@colour} #{@model} car with #{@seat_count} seats and #{@door_count} doors."
    end
  end

  class CarBuilder
    private getter car : Car

    def initialize
      @car = Car.new
    end

    def set_seat_count(seat_count)
      @car.seat_count = seat_count
    end

    def set_door_count(door_count)
      @car.door_count = door_count
    end

    def set_colour(colour)
      @car.colour = colour
    end

    def set_model(model)
      @car.model = model
    end

    def construct
      @car
    end
  end
end
