-- Init the class
local Paddle = {}
Paddle.__index = Paddle

-- Create a new paddle
function Paddle:new(x, y, width, height, up_key, down_key,
                    top_limit, bottom_limit ,speed)
  local new_paddle ={}
  setmetatable(new_paddle, Paddle)
  -- Set the Paddle properties to the ones passed or to the default
  new_paddle.x = x or 0 
  new_paddle.y = y or 0
  new_paddle.width = width or 0
  new_paddle.height = height or 0
  -- Obligatory parameters
  assert(up_key and down_key,'Incomplete class parameters, keys are obligatory')
  new_paddle.up_key = up_key
  new_paddle.down_key = down_key
  -- Limit in which the paddle can move
  new_paddle.top_limit = top_limit or -math.huge
  new_paddle.bottom_limit = bottom_limit - new_paddle.height or math.huge
  new_paddle.speed = speed or 200
  -- Fixed properties
  new_paddle.velocity = 0
  return new_paddle
end

-- Update the values of the paddle
function Paddle:update(dt)
  -- Reset the paddle velocity
  self.velocity = 0
  -- Check if any of the key is pressed
  if love.keyboard.isDown(self.up_key) then
    self.velocity = self.velocity - (self.speed * dt)
  end
  if love.keyboard.isDown(self.down_key) then
    self.velocity = self.velocity + (self.speed * dt)
  end
  -- Update paddle position
  if self.velocity < 0 then
    self.y = math.max(self.y + self.velocity, self.top_limit)
  elseif self.velocity > 0 then
    self.y = math.min(self.y + self.velocity, self.bottom_limit)
  end
end

-- Draw the paddle
function Paddle:draw()
  love.graphics.rectangle('fill', self.x, self.y, self.width, self.height)
end

return Paddle