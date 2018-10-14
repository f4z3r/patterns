# Decorator pattern

module Patterns
  abstract class Coffee
    abstract def cost
    abstract def ingredients
  end

  class SimpleCoffee < Coffee
    def cost
      1.0
    end

    def ingredients
      "coffee"
    end
  end

  # Abstract decorator
  abstract class CoffeeDecorator < Coffee
    protected getter decorated_coffee : Coffee

    def initialize(@decorated_coffee)
    end

    def cost
      @decorated_coffee.cost
    end

    def ingredients
      @decorated_coffee.ingredients
    end
  end

  # Concrete decorator
  class WithMilk < CoffeeDecorator
    def cost
      super + 0.5
    end

    def ingredients
      super + ", milk"
    end
  end

  # Concrete decorator
  class WithChocolate < CoffeeDecorator
    def cost
      super + 0.3
    end

    def ingredients
      super + ", chocolate"
    end
  end
end
