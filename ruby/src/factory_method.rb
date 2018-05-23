#!/usr/bin/env ruby -w

# The factory method design pattern.
#
# Creates a sort of "virtual" constructor. Allows clients to interact with objects without knowing their true type.


# Base class of the constructed object
class Car
  def drive
    "wrooom"
  end
end

# Derived class of the constructed object
class Sedan < Car
end

# Derived class of the constructed object
class Hatchback < Car
end

# Mixin to add to concrete creator object
module Creator
  def make_car
    Car.new()     # Provide default implementation. Could also raise NotImplementedError
  end
end

# Concrete creator implementing the Creator mixin
class SedanFactory
  include Creator
  def make_car
    Sedan.new()
  end
end

# Concrete creator implementing the Creator mixin
class HatchbackFactory
  include Creator
  def make_car
    Hatchback.new()
  end
end




