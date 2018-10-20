# Abstract factory pattern

module Patterns::AbstractFactory
  # Abstract entity to be created by the factory.
  abstract class Button
    abstract def paint: String
  end

  # Interface of the factory. Defines the factory methods.
  abstract class Factory
    abstract def create_button: Button
  end

  # Concrete entity created by a factory.
  class OSXButton < Button
    def paint
      "osx button"
    end
  end

  # Concrete entity created by a factory.
  class WindowsButton < Button
    def paint
      "windows button"
    end
  end

  # Concrete factory creating objects.
  class OSXFactory < Factory
    def create_button
      OSXButton.new
    end
  end

  # Concrete factory creating objects.
  class WindowsFactory < Factory
    def create_button
      WindowsButton.new
    end
  end
end
