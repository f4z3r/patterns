require "./spec_helper"

describe Patterns do
  it "factory method works" do
    scary_maze = Patterns::FactoryMethod::ScaryMaze.new
    scary_maze.print.should eq "a very scarrrrryyy room;a very scarrrrryyy room;"
  end

  it "abstract factory works" do
    os_type = "OSX"
    factory = nil
    case os_type
    when "Windows"
      factory = Patterns::AbstractFactory::WindowsFactory.new
    when "OSX"
      factory = Patterns::AbstractFactory::OSXFactory.new
    end

    unless factory.nil?
      button = factory.create_button
      button.paint.should eq "osx button"
    end
  end

  it "decorator works" do
    coffee = Patterns::Decorator::WithChocolate.new(
      Patterns::Decorator::WithMilk.new(Patterns::Decorator::SimpleCoffee.new))
    coffee.cost.should eq 1.8
    coffee.ingredients.should eq "coffee, milk, chocolate"
  end

  it "builder works" do
    builder = Patterns::Builder::CarBuilder.new
    builder.set_seat_count(7_u8)
    builder.set_door_count(4_u8)
    builder.set_model("sedan")
    builder.set_colour("red")
    car = builder.construct
    car.description.should eq "red sedan car with 7 seats and 4 doors."
  end

  it "state works" do
    post = Patterns::State::Post.new
    post.add_text("This is my amazing post.")
    post.get_content.should eq ""
    post.request_review
    post.get_content.should eq ""
    post.approve
    post.get_content.should eq "This is my amazing post."
    post.add_text(" And I made some changed to make it even more awesome.")
    post.get_content.should eq ""
    post.request_review
    post.approve
    post.get_content.should eq "This is my amazing post. And I made some changed to make it even more awesome."
  end

  it "command works" do
    client = Patterns::Command::Client.new
    client.switch_for("on")
    client.switch_for("off")
    client.switch.history.should eq ["light turned on", "light turned off"]
    expect_raises(ArgumentError, "unsupported argument 'else'") do
      client.switch_for("else")
    end
  end

  it "strategy works" do
    obj = Patterns::Strategy::SomeObject.new(Patterns::Strategy::FastAlgorithm.new)
    obj.print.should eq "running a very fast algorithm"
  end

  it "adapter works" do
    iphone = Patterns::Adapter::IPhone.new
    # cannot call iphone.recharge
    adapter = Patterns::Adapter::USBCharger.new(iphone)
    adapter.recharge.should eq "adding adapter and charging iPhone"
  end

  it "template method works" do
    obj = Patterns::TemplateMethod::DoSomethingSpecific.new
    obj.do_something.should eq "starting do specific thing 1; doing thing 2 and do generic thing 3"
  end

  it "facade pattern works" do
    compiler = Patterns::Facade::Compiler.new
    compiler.run.should eq "parsing source code\ngenerating machine code\noptimising generated machine code\nlinking code\n"
  end
end
