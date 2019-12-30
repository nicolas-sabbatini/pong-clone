-- Load modules
local push   = require('push')
local Paddle = require('Paddle')
local Ball   = require('Ball')

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
  score_font = love.graphics.newFont('assets/fonts/fff-forward/FFFFORWA.TTF', 24)
  -- Set the push library
  local win_width, win_height = love.graphics.getDimensions()
  push:setupScreen(FAKE_WIDTH, FAKE_HEIGHT, win_width, win_height)
  -- Create players
  players = {}
  players[1] = Paddle:new(15, (FAKE_HEIGHT/2) - 12, 6, 24,
                         'w', 's', 0, FAKE_HEIGHT)
  players[2] = Paddle:new(FAKE_WIDTH - 21, (FAKE_HEIGHT/2) - 12, 6, 24,
                         'up', 'down', 0, FAKE_HEIGHT)
  -- Scores
  score = {0, 0}
  -- Create the ball
  ball = Ball:new((FAKE_WIDTH/2) - 5, (FAKE_HEIGHT/2) - 5, 10, 10, 15)
  -- Set game state
  state = 'ready'
  -- Select to witch player we need to serve
  serve_to = nil 
end

function love.update(dt)
  -- An exit key
  if love.keyboard.isDown('escape') then love.event.quit() end
  -- Start the ball if the game is in the corect state
  if love.keyboard.isDown('space') and (state == 'waiting' or
                                        state == 'ready'   or 
                                        state == 'win'    )then
    ball:start(serve_to)
    if state == 'win' then
      score = {0, 0}
    end
    state = 'play'
  end
  -- Update the players position
  for k, player in pairs(players) do
    player:update(dt)
  end
  -- Ball Collition
  -- Top of the screen
  if ball:collisionAABB(0, -100, FAKE_WIDTH, 100) then 
    ball.velocity_y = -ball.velocity_y
    ball.y = 1
  end
  -- Botom of the screen
  if ball:collisionAABB(0, FAKE_HEIGHT, FAKE_WIDTH, 100)  then 
    ball.velocity_y = -ball.velocity_y
    ball.y = FAKE_HEIGHT - ball.height - 1
  end
  -- Players
  if ball:collisionAABB(players[1].x,     players[1].y,
                        2, players[1].height) then
    ball.velocity_x = - (ball.velocity_x - ball.acceleration)
    ball.velocity_y = 120 * (love.math.random() * 2 - 1)
    ball.x = players[1].x + players[1].width + 1
  end
  if ball:collisionAABB(players[2].x,     players[2].y,
                        2, players[2].height) then
    ball.velocity_x = - (ball.velocity_x + ball.acceleration)
    ball.velocity_y = 120 * (love.math.random() * 2 - 1)
    ball.x = players[2].x - ball.width - 1
  end
  -- The end of the screen
  -- Lefth
  if ball:collisionAABB(-100, 0, 100, FAKE_HEIGHT) then 
    ball:reset()
    serve_to = -1
    score[2] = score[2] + 1
    state = score[2] > 10 and 'waiting' or 'win'
  end
  -- Right
  if ball:collisionAABB(FAKE_WIDTH, 0, 100, FAKE_HEIGHT) then 
    ball:reset()
    serve_to = 1
    score[1] = score[1] + 1
    state = score[1] > 10 and 'waiting' or 'win'
  end
  ball:update(dt)
end

function love.draw()
  -- Push resolution
  push:apply('start')
  if state == 'ready' then
    love.graphics.setFont(ui_font)
    love.graphics.printf('READY?', 0, 25, FAKE_WIDTH, 'center')
  elseif state == 'waiting' then
    love.graphics.setFont(ui_font)
    love.graphics.printf('PLAYER ' .. (serve_to == 1 and 1 or 2) .. ' SCORED',
                         0, 25, FAKE_WIDTH, 'center')
  elseif state == 'win' then
    love.graphics.setFont(ui_font)
    love.graphics.printf('PLAYER ' .. (serve_to == 1 and 1 or 2) ..
                         ' WINS THE GAME', 0, 25, FAKE_WIDTH, 'center')  
  end
  -- Draw scores
  love.graphics.setFont(score_font)
  love.graphics.printf(score[1],            0, 20, FAKE_WIDTH / 2, 'center')
  love.graphics.printf(score[2], FAKE_WIDTH/2, 20, FAKE_WIDTH / 2, 'center')
  -- Drw players
  for k, player in pairs(players) do
    player:draw(dt)
  end
  -- Draw Ball
  ball:draw()
  -- Pop resolution
  push:apply('end')
end
