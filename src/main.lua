-- Load modules
local push = require('push')
local Paddle = require('Paddle')

-- The resolution in witch the game is going to be render
FAKE_WIDTH = 432
FAKE_HEIGHT = 243

-- This function is caled on the start of the game
function love.load()
  -- Set random seed
  love.math.setRandomSeed(os.time())
  -- Set the scaling filter
  love.graphics.setDefaultFilter('nearest', 'nearest')
  -- Load font
  ui_font = love.graphics.newFont('assets/fonts/fff-forward/FFFFORWA.TTF', 8)
  score_font = love.graphics.newFont('assets/fonts/fff-forward/FFFFORWA.TTF', 40)
  -- Set 'font' as default
  love.graphics.setFont(score_font)
  -- Set the push library
  local win_width, win_height = love.graphics.getDimensions()
  push:setupScreen(FAKE_WIDTH, FAKE_HEIGHT, win_width, win_height)
  -- Create palyers
  palyers = {}
  palyers[1] = Paddle:new(15, (FAKE_HEIGHT/2) - 12, 6, 24,
                         'w', 's', 0, FAKE_HEIGHT)
  palyers[2] = Paddle:new(FAKE_WIDTH - 21, (FAKE_HEIGHT/2) - 12, 6, 24,
                         'up', 'down', 0, FAKE_HEIGHT)
end

function love.update(dt)
  -- An exit key
  if love.keyboard.isDown('escape') then love.event.quit() end
  for k, player in pairs(palyers) do
    player:update(dt)
  end
end

function love.draw()
  -- Push resolution
  push:apply('start')
  for k, player in pairs(palyers) do
    player:draw(dt)
  end
  -- Pop resolution
  push:apply('end')
end
