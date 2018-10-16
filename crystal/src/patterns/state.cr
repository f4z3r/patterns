# State design pattern

module Patterns
  # abstract state class
  abstract class State
    abstract def request_review : State
    abstract def approve : State
    def content(post : Post)
      ""
    end
  end

  # concrete state
  class Draft < State
    def request_review
      PendingReview.new
    end

    def approve
      self
    end
  end

  # concrete state
  class PendingReview < State
    def request_review
      self
    end

    def approve
      Published.new
    end
  end

  # concrete state
  class Published < State
    def request_review
      PendingReview.new
    end

    def approve
      self
    end

    def content(post : Post)
      post.content
    end
  end

  # class having state
  class Post
    property state : State
    protected property content : String

    def initialize(@state = Draft.new, @content = "")
    end

    def add_text(text)
      @content += text
      @state = Draft.new
    end

    def request_review
      @state = @state.request_review
    end

    def approve
      @state = @state.approve
    end

    def get_content
      @state.content(self)
    end
  end
end
