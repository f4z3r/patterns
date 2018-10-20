require "./spec_helper"

describe Patterns do
  it "factory method works" do
    scary_maze = Patterns::ScaryMaze.new
    scary_maze.print.should eq "a very scarrrrryyy room;a very scarrrrryyy room;"
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
      button.paint.should eq "osx button"
    end
  end

  it "decorator works" do
    coffee = Patterns::WithChocolate.new(Patterns::WithMilk.new(Patterns::SimpleCoffee.new))
    coffee.cost.should eq 1.8
    coffee.ingredients.should eq "coffee, milk, chocolate"
  end

  it "builder works" do
    builder = Patterns::CarBuilder.new
    builder.set_seat_count(7_u8)
    builder.set_door_count(4_u8)
    builder.set_model("sedan")
    builder.set_colour("red")
    car = builder.construct
    car.description.should eq "red sedan car with 7 seats and 4 doors."
  end

  it "state works" do
    post = Patterns::Post.new
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
    client = Patterns::Client.new
    client.switch_for("on")
    client.switch_for("off")
    client.switch.history.should eq ["light turned on", "light turned off"]
    expect_raises(ArgumentError, "unsupported argument 'else'") do
      client.switch_for("else")
    end
  end

  it "strategy works" do
    obj = Patterns::SomeObject.new(Patterns::FastAlgorithm.new)
    obj.print.should eq "running a very fast algorithm"
  end

  it "adapter works" do
    iphone = Patterns::IPhone.new
    # cannot call iphone.recharge
    adapter = Patterns::USBCharger.new(iphone)
    adapter.recharge.should eq "adding adapter and charging iPhone"
  end

  it "template method works" do
    obj = Patterns::DoSomethingSpecific.new
    obj.do_something.should eq "starting do specific thing 1; doing thing 2 and do generic thing 3"
  end
end
