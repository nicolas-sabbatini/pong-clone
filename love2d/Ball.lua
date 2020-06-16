local Ball = {}
Ball.__index = Ball

-- Create a new Ball
function Ball:new(x, y, width, height, acceleration)
  local new_ball = {}
  setmetatable(new_ball, Ball)
  -- Set the Ball properties to the ones passed or to the default
  new_ball.x = x or 0
  new_ball.y = y or 0
  new_ball.width = width or 0
  new_ball.height = height or 0
  new_ball.acceleration = acceleration or 10
  new_ball.velocity_x = 0
  new_ball.velocity_y = 0
  -- Set the default reset values
  new_ball.default_x = new_ball.x
  new_ball.default_y = new_ball.y
  return new_ball
end

-- Update the Position of the ball
function Ball:update(dt)
  self.x = self.x + (self.velocity_x * dt)
  self.y = self.y + (self.velocity_y * dt)
end

-- Start the ball
function Ball:start(side)
  assert(side == 1 or side == -1 or side == nil,
        'Ball:start only acepts values 1 or -1 or nil')
  -- Selet witch side the ball is goin to
  side = side or (love.math.random() < 0.5 and 1 or -1)
  self.velocity_x =  50 * side
  self.velocity_y = 100 * (love.math.random() * 2 - 1) 
end

-- Reset the ball position and velocity
function Ball:reset()
  self.x = self.default_x
  self.y = self.default_y
  self.velocity_x = 0
  self.velocity_y = 0
end

-- Draw the ball
function Ball:draw()
  love.graphics.rectangle('fill', self.x, self.y, self.width, self.height)
end

-- Check if the ball is collides with a given rectangle
function Ball:collisionAABB(top_left_x, top_left_y, width, height)
  if( self.x               < top_left_x + width  and
      self.x + self.width  > top_left_x          and
      self.y               < top_left_y + height and
      self.y + self.height > top_left_y             ) then
    return true 
  end
  return false
end
return Ball