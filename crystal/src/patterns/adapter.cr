# Adapter design pattern

module Patterns::Adapter
  # target interface the client uses
  module TargetInterface
    abstract def recharge
  end

  # the adaptee
  class IPhone
    def charge : String
      "charging iPhone"
    end
  end

  # the adapter
  class USBCharger
    include TargetInterface
    getter iphone : IPhone

    def initialize(@iphone = IPhone.new)
    end

    def recharge : String
      "adding adapter and " + @iphone.charge
    end
  end
end
