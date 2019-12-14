-- Init the class
local Paddle = {}
Paddle.__index = Paddle

function Paddle:new(x, y, width, height, top_limit, bottom_limit ,speed)
  local new_paddle ={}
  setmetatable(new_paddle, Paddle)
  -- Set the Paddle properties to the ones passed or to the default
  new_paddle.x = x or 0 
  new_paddle.y = y or 0
  new_paddle.width = width or 0
  new_paddle.height = height or 0
  -- Limit in which the paddle can move
  new_paddle.top_limit = top_limit or -math.huge
  new_paddle.bottom_limit = bottom_limit -  or math.huge
  new_paddle.speed = speed or 200
  -- Fixed properties
  new_paddle.velocity = 0
  return new_paddle
end

-- Update the values of the object
function Paddle:update(dt)
  if self.velocity < 0 then
    self.y = math.max(self.y - self.velocity, self.top_limit)
  else if self.velocity > 0 then
    self.y = math.min(self.y + self.velocity + self.height, self.bottom_limit)
  end
end

-- Draw the object
function Paddle:draw()
  love.graphics.rectangle('fill', self.x, self.y, self.width, self.height)
end

return Paddle