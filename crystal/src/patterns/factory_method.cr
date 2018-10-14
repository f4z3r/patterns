# Factory method pattern.

module Patterns
  # Base class instantiated by the factory method
  class Room
    def print
      "a room;"
    end
  end

  class Maze
    def initialize
      @rooms = [] of Room
      @rooms << self.make_room
      @rooms << self.make_room
    end

    def print
      result = ""
      @rooms.each do |room|
        result += room.print
      end
      result
    end

    # The factory method. This method should be overriden in subclasses to allow the initialiser to customise the maze
    # according using this "virtual constructor".
    def make_room
      Room.new
    end
  end

  # Subclass of the room type used in the regular maze.
  class ScaryRoom < Room
    def print
      "a very scarrrrryyy room;"
    end
  end

  class ScaryMaze < Maze
    # Override of the factory method to modify the rooms used in the maze constructor.
    def make_room
      ScaryRoom.new
    end
  end
end
