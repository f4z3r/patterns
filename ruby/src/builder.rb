#!/usr/bin/env ruby -w

# Builder design pattern
#
# Solves the problem of construction of very complex objects using incremental construction.


class Car
  attr_accessor :wheels, :seats, :doors
  def initialize
    @wheels = 0
    @seats = 0
    @doors = 0
  end
end

module Builder
  def set_wheels(wheels)
    raise NotImplementedError, "not implemented"
  end
  def set_doors(doors)
    raise NotImplementedError, "not implemented"
  end
  def set_seats(seats)
    raise NotImplementedError, "not implemented"
  end
  def build
    raise NotImplementedError, "not implemented"
  end
end

class CarBuilder
  attr_reader :car
  include Builder
  def initialize
    @car = Car.new
  end
  def set_wheels(wheels)
    @car.wheels = wheels
  end
  def set_doors(doors)
    @car.doors = doors
  end
  def set_seats(seats)
    @car.seats = seats
  end
  def build
    return @car
  end
end


