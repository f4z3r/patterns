#!/usr/bin/env ruby -w

# The abstract factory design pattern.
#
# This is basically a collection of factory methods implemented on a single class.


# Button base class. Its derived classes are built by concrete factories.
class Button
  def click
    "clicked"
  end
end

class Window
  def get_height
    100
  end
end

# Concrete button class derived from the `Button` base class.
class LinuxButton < Button
end

class LinuxWindow < Window
  def get_height
    200
  end
end

class OSXButton < Button
  def click
    "clocked"
  end
end

class OSXWindow < Window
end

# Factory mixin
module Factory
  def make_button
    Button.new()    # Provides default implementation
  end
  def make_window
    Window.new()    # Provides default implementation
  end
end

# Concrete factory.
class LinuxFactory
  include Factory
  def make_button
    LinuxButton.new()
  end
  def make_window
    LinuxWindow.new()
  end
end

# Concrete factory.
class OSXFactory
  include Factory
  def make_button
    OSXButton.new()
  end
  def make_window
    OSXWindow.new()
  end
end



