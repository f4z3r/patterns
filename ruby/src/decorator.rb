#!/usr/bin/env ruby -w

# Decorator design pattern
#
# Allows to dynamically attach additional responsabilities to an object.

module Shape
  def description
    raise NotImplementedError, "not implemented"
  end
end

class Circle
  include Shape
  def description
    "circle"
  end
end

class ColouredShape
  include Shape
  def initialize(object, colour)
    @object = object
    @colour = colour
  end
  def description
    "#{@colour} #{@object.description}"
  end
end
