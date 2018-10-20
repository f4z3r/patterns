# Template method design pattern

module Patterns::TemplateMethod
  # the abstract class defining the template (`do_something`) that is customisable via overriding the abstract methods
  abstract class DoSomething
    def do_something : String
      "starting " + do_thing_1 + "; " + do_thing_2 + " and " + do_thing_3
    end

    abstract def do_thing_1 : String

    private def do_thing_2 : String
      "doing thing 2"
    end

    abstract def do_thing_3 : String
  end

  # the concrete class implementing the template
  class DoSomethingSpecific < DoSomething
    def do_thing_1 : String
      "do specific thing 1"
    end

    def do_thing_3 : String
      "do generic thing 3"
    end
  end
end
