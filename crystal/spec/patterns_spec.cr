require "./spec_helper"

describe Patterns do
  it "factory method works" do
    scary_maze = Patterns::ScaryMaze.new
    scary_maze.print.should eq("a very scarrrrryyy room;a very scarrrrryyy room;")
  end

  it "abstract factory works" do
    os_type = "OSX"
    factory = nil
    case os_type
    when "Windows"
      factory = Patterns::WindowsFactory.new
    when "OSX"
      factory = Patterns::OSXFactory.new
    end

    unless factory.nil?
      button = factory.create_button
      button.paint.should eq("osx button")
    end
  end

  it "decorator works" do
    coffee = Patterns::WithChocolate.new(Patterns::WithMilk.new(Patterns::SimpleCoffee.new))
    coffee.cost.should eq(1.8)
    coffee.ingredients.should eq("coffee, milk, chocolate")
  end

  it "builder works" do
    builder = Patterns::CarBuilder.new
    builder.set_seat_count(7_u8)
    builder.set_door_count(4_u8)
    builder.set_model("sedan")
    builder.set_colour("red")
    car = builder.construct
    car.description.should eq("red sedan car with 7 seats and 4 doors.")
  end
end
