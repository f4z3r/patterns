# command design pattern

module Patterns::Command
  # the command executor which stores a command history
  class Switch
    getter history : Array(String)

    def initialize
      @history = [] of String
    end

    def execute(command : Command)
      @history << command.execute
    end
  end

  # abstract command superclass
  abstract class Command
    getter obj : Light

    def initialize(@obj)
    end

    abstract def execute
  end

  # concrete command
  class TurnOnCommand < Command
    def execute
      @obj.turn_on
    end
  end

  # other concrete command
  class TurnOffCommand < Command
    def execute
      @obj.turn_off
    end
  end

  # receiver of the command
  class Light
    def turn_on
      "light turned on"
    end

    def turn_off
      "light turned off"
    end
  end

  class Client
    getter switch : Switch

    def initialize
      @light = Light.new
      @switch = Switch.new
    end

    def switch_for(cmd : String) : Nil
      case cmd
      when "on"   then @switch.execute(TurnOnCommand.new(@light))
      when "off"  then @switch.execute(TurnOffCommand.new(@light))
      else raise ArgumentError.new("unsupported argument '#{cmd}'")
      end
    end
  end
end
