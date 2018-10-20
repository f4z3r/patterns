# Strategy design pattern

module Patterns::Strategy
  # algorithm interface
  module Algorithm
    abstract def run : String
  end

  # concrete algorithm
  class FastAlgorithm
    include Algorithm

    def run : String
      "very fast algorithm"
    end
  end

  # concrete algorithm
  class SlowAlgorithm
    include Algorithm

    def run : String
      "very slow algorithm"
    end
  end

  # object using an attached algorithm
  class SomeObject
    getter algo : Algorithm

    def initialize(@algo : Algorithm)
    end

    def print : String
      "running a #{@algo.run}"
    end
  end
end
