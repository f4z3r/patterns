# Facade design pattern

module Patterns::Facade
  class Linker
    def run : String
      "linking code\n"
    end
  end

  class Parser
    def run : String
      "parsing source code\n"
    end
  end

  class Optimiser
    def run : String
      "optimising generated machine code\n"
    end
  end

  class CodeGenerator
    def run : String
      "generating machine code\n"
    end
  end

  class Compiler
    getter parser : Parser
    getter generator : CodeGenerator
    getter optimiser : Optimiser
    getter linker : Linker

    def initialize
      @parser = Parser.new
      @generator = CodeGenerator.new
      @optimiser = Optimiser.new
      @linker = Linker.new
    end

    def run : String
      result = ""
      result += @parser.run
      result += @generator.run
      result += @optimiser.run
      result += @linker.run
      result
    end
  end
end
