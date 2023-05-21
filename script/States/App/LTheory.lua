local Player = require('GameObjects.Entities.Player')
local System = require('GameObjects.Entities.Test.System')
local DebugControl = require('Systems.Controls.Controls.DebugControl')
local Actions = requireAll('GameObjects.Actions')
local Bindings = require('States.ApplicationBindings')

local LTheory = require('States.Application')
local rng = RNG.FromTime()

local ships = {}
local escortShips = 100

function LTheory:generate ()
  self.seed = rng:get64()
  if true then
    -- self.seed = 7035008865122330386ULL
    -- self.seed = 15054808765102574876ULL
    -- self.seed = 1777258448479734603ULL
    -- self.seed = 5023726954312599969ULL
  end
  printf('Seed: %s', self.seed)

  if self.system then self.system:delete() end
  self.system = System(self.seed)

  GameState.world.currentSystem = self.system
  GameState.ui.hudStyle = Enums.HudStyles.Wide
  GameState:SetState(Enums.GameStates.InGame)

  local shipSize = Enums.ShipHulls.VeryLarge
--  local shipSize = Enums.ShipHulls.Solo

  local ship
  ship = self.system:spawnShip(shipSize, self.player)
  ship:setName(GameState.player.humanPlayerShipName)
  ship:setPos(Config.gen.origin)
  if shipSize == Enums.ShipHulls.VeryLarge then
    ship:setScale(0.7)
  end
  ship:setFriction(0)
  ship:setSleepThreshold(0, 0)
  ship:setOwner(self.player)
  self.system:addChild(ship)
  self.player:setControlling(ship)
  GameState.player.currentShip = ship

  for i = 1, escortShips do
    shipSize = rng:choose({Enums.ShipHulls.Solo,
                           Enums.ShipHulls.Small,
                           Enums.ShipHulls.Compact,
                           Enums.ShipHulls.Medium,
                           Enums.ShipHulls.Large,
                           Enums.ShipHulls.VeryLarge})
    local escort = self.system:spawnShip(shipSize, nil)
    local offset = rng:getSphere():scale(300)
    escort:setPos(ship:getPos() + offset)
    escort:setOwner(self.player)
    if rng:getInt(0, 100) < 20 then
      escort.usesBoost = true
    end
    escort:pushAction(Actions.Escort(ship, offset))
    insert(ships, escort)
  end

  for i = 1, #ships - 1 do
    ships[i]:pushAction(Actions.Attack(ships[i+1]))
  end

  for i = 1, 1 do
    local station = self.system:spawnStation(Enums.StationHulls.Small, nil, nil)
  end

--  for i = 1, 0 do
--    self.system:spawnAI(100, Actions.Wait(5), self.player)
--  end

  for i = 1, 10 do
    self.system:spawnAsteroidField(200, 10)
  end

  for i = 1, 1 do
    self.system:spawnPlanet()
  end
end

function LTheory:onInit ()
  --* Audio initializations *--
  Audio.Init()
  Audio.Set3DSettings(0.0, 10, 2);

  self.player = Player("LTheory Player")
  self:generate()

  DebugControl.ltheory = self
  self.gameView = Systems.Overlay.GameView(self.player)
  self.canvas = UI.Canvas()
  self.canvas
    :add(self.gameView
      :add(Systems.Controls.Controls.MasterControl(self.gameView, self.player)))
end

function LTheory:onInput ()
  self.canvas:input()
end

function LTheory:onUpdate (dt)
  -- If player pressed the "ToggleLights" key in Flight Mode, toggle dynamic lighting on/off
  -- NOTE: Performance is OK for just the player's ship, but adding many lit ships & pulses tanks performance
  if Input.GetPressed(Bindings.ToggleLights) then
    GameState.render.pulseLights = not GameState.render.pulseLights
  end

  self.player:getRoot():update(dt)
  self.canvas:update(dt)

  HmGui.Begin(self.resX, self.resY) -- required for HmGui.Draw() to work without crashing
  HmGui.End()
end

function LTheory:onDraw ()
  self.canvas:draw(self.resX, self.resY)
  HmGui.Draw() -- post-Rust, required for game universe to be displayed
end

return LTheory
