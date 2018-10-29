# Flyweight design pattern

module Patterns::Flyweight
  # flyweight object. Is shared across all cheeseshops.
  class Menu
    property items : Hash(String, CheeseBrand)

    def initialize
      @items = Hash(String, CheeseBrand).new
    end

    def add(name, cost, quantity)
      cheesebrand = CheeseBrand.new(name, cost, quantity)
      @items[name] = cheesebrand
    end

    def sell(name, quantity) : Float64
      if @items[name].quantity >= quantity
        @items[name].quantity -= quantity
        return quantity * @items[name].cost
      end
      -1.0
    end
  end

  class CheeseBrand
    property name : String
    property cost : Float64
    property quantity : Float64

    def initialize(@name, @cost, @quantity)
    end
  end

  # class sharing the flyweight object
  class CheeseShop
    property menu : Menu
    getter units_sold : Float64
    getter revenue : Float64

    def initialize(@menu)
      @units_sold = 0
      @revenue = 0
    end

    def stock_cheese(name, cost, quantity)
      @menu.add(name, cost, quantity)
    end

    def sell(name, quantity)
      money = @menu.sell(name, quantity)
      if money != -1.0
        @units_sold += quantity
        @revenue += money
      end
    end
  end
end
