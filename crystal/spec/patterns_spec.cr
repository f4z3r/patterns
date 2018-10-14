require "./spec_helper"

describe Patterns do
  # TODO: Write tests

  it "factory method works" do
    scary_maze = Patterns::ScaryMaze.new
    scary_maze.print.should eq("a very scarrrrryyy room;a very scarrrrryyy room;")
  end
end
