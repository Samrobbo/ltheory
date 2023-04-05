local DebugContext = require('Systems.CommandView.DebugContext')
local Bindings = require('States.ApplicationBindings')
local Player = require('GameObjects.Entities.Player')

local SystemMap = {}
SystemMap.__index  = SystemMap
setmetatable(SystemMap, UI.Container)

local kPanSpeed = 20 -- NOTE: may be dependent on player's CPU, needs testing
local kZoomSpeed = 0.1

SystemMap.scrollable = true
SystemMap.focusable  = true
SystemMap:setPadUniform(0)

function SystemMap:onDraw (state)
  Draw.Color(0.1, 0.11, 0.12, 1)
  local x, y, sx, sy = self:getRectGlobal()
  Draw.Rect(x, y, sx, sy)

  Draw.Color(0, 1, 0, 1)
  local hx, hy = sx / 2, sy / 2
  local dx, dy = Config.game.mapSystemPos.x + hx, Config.game.mapSystemPos.y + hy

  local c = {
    r = 0.1,
    g = 0.5,
    b = 1.0,
    a = 0.1,
  }

  local best = nil
  local bestDist = math.huge
  local mp = Input.GetMousePosition()

  -- If an object is target locked in flight view (via HUD), give it focus in the System Map
  local playerShip = Config.game.currentShip
  local playerTarget = nil
  if playerShip ~= nil then
    if self.focus == playerShip then
      playerShip:setTarget(nil)
    end
    playerTarget = playerShip:getTarget()
  end
  if playerTarget ~= nil then
--printf("Targeting a %s", Config:getObjectInfo("object_types", playerTarget:getType()))
    self.focus = playerTarget
  end

  BlendMode.PushAlpha()
  Draw.SmoothPoints(true)
--printf("------------------------------")
  for _, e in self.system:iterChildren() do
    -- Check to make sure this is an actual object with a body
    if e.body ~= nil then
--printf("Drawing %s '%s'", Config.objectInfo[1]["elems"][e:getType()][2], e:getName())
      local p = e:getPos()
      local x = p.x - dx
      local y = p.z - dy
      x = self.x + x * Config.game.mapSystemZoom + hx
      y = self.y + y * Config.game.mapSystemZoom + hy
      Draw.PointSize(3.0)

      if e:hasActions() then
--printf("Action: %s", e:getName())
        if Config.game.currentShip == e then
          Draw.PointSize(5.0)
          Draw.Color(0.9, 0.5, 1.0, 1.0) -- player ship
          if playerTarget then
            local tp = playerTarget:getPos()
            local tx = tp.x - dx
            local ty = tp.z - dy
            tx = self.x + tx * Config.game.mapSystemZoom + hx
            ty = self.y + ty * Config.game.mapSystemZoom + hy
            UI.DrawEx.Line(x, y, tx, ty, { r = 0.9, g = 0.8, b = 1.0, a = 1.0 })
          end
        else
          local entAction = e:getCurrentAction()
          if entAction ~= nil then
--printf("Action is '%s', target is '%s'", entAction:getName(), entAction.target:getName())
            if string.match(Config:getObjectInfo("object_types", e:getType()), "Ship") and e.usesBoost then
              -- Draw the dot for ships that are aces larger than regular ships
              Draw.PointSize(5.0)
            end
            if string.find(entAction:getName(), "Attack") and entAction.target == Config.game.currentShip then
              -- TODO: draw in color based on Disposition toward player
              Draw.Color(1.0, 0.3, 0.3, 1.0) -- other object, hostile (has a current action of "Attack player's ship")
            else
              Draw.Color(0.2, 0.6, 1.0, 1.0) -- other object, non-hostile
            end
            local focusedTarget = e:getTarget()
            if focusedTarget then
              local ftp = focusedTarget:getPos()
              local ftx = ftp.x - dx
              local fty = ftp.z - dy
              ftx = self.x + ftx * Config.game.mapSystemZoom + hx
              fty = self.y + fty * Config.game.mapSystemZoom + hy
              if string.find(entAction:getName(), "Attack") then
                UI.DrawEx.Line(x, y, ftx, fty, { r = 1.0, g = 0.4, b = 0.3, a = 1.0 })
              else
                UI.DrawEx.Line(x, y, ftx, fty, { r = 1.0, g = 1.0, b = 1.0, a = 1.0 })
              end
            end
          else
            Draw.Color(1.0, 1.0, 1.0, 1.0) -- some other object that suddenly has no actions
          end
        end
      else
        Draw.Color(0.4, 0.4, 0.4, 1.0) -- planet, asteroid, station
      end
      Draw.Point(x, y)

      if e:hasFlows() and not e:isDestroyed() then
--printf("Flow: %s", e:getName())
        UI.DrawEx.Ring(x, y, Config.game.mapSystemZoom * e:getScale() * 10, { r = 0.1, g = 0.5, b = 1.0, a = 1.0 })
      end

      if e:hasYield() then
--printf("Yield: %s", e:getName())
        UI.DrawEx.Ring(x, y, Config.game.mapSystemZoom * e:getScale(), { r = 1.0, g = 0.5, b = 0.1, a = 0.5 })
      end

      if self.focus == e then
--printf("Focus: %s", e:getName())
        UI.DrawEx.Ring(x, y, 8, { r = 1.0, g = 0.0, b = 0.3, a = 1.0 })
      end

      -- Select the nearest object
      local d = Vec2f(x, y):distanceSquared(mp)
      if d < bestDist then
        bestDist = d
        best = e
      end
--    else
--      -- Non-object entities (e.g., zones)
--printf("Found %s '%s'", Config.objectInfo[1]["elems"][e:getType()][2], e:getName())
--      local p = e:getPos()
--      local x = p.x - dx
--      local y = p.z - dy
--      x = self.x + x * Config.game.mapSystemZoom + hx
--      y = self.y + y * Config.game.mapSystemZoom + hy
--      Draw.PointSize(2.0)
--      Draw.Color(1.0, 1.0, 1.0, 1)
--      Draw.Point(x, y)
--      --UI.DrawEx.Ring(x, y, Config.game.mapSystemZoom * e:getScale(), { r = 0.8, g = 0.3, b = 0.8, a = 0.7 })
    end
  end
  Draw.Color(1, 1, 1, 1)
  Draw.SmoothPoints(false)
  BlendMode.Pop()

  if Input.GetDown(Button.Mouse.Left) then
    self.focus = best
    -- Set focused-on object in the System Map as the player ship's current target
    if Config.game.currentShip ~= nil and Config.game.currentShip ~= self.focus then
      Config.game.currentShip:setTarget(self.focus)
    end
  end

  do -- Debug Info
    local dbg = DebugContext(16, 16)
    dbg:text("--- System ---")
    dbg:indent()
    self.system:send(Event.Debug(dbg))
    dbg:undent()

    if self.focus then
      local objtype    = Config:getObjectInfo("object_types", self.focus:getType())
      local objsubtype = Config:getObjectSubInfo("object_types", self.focus:getType(), self.focus:getSubType())
      local owner = self.focus:getOwner()
      local objval = 0
      local objemit = ""
      local boomtext = ""
      local acetext = ""
      if self.focus:isDestroyed() then boomtext = " (destroyed)" end
      if string.match(objtype, "Ship") and self.focus.usesBoost then acetext = " [Ace]" end
      dbg:text("")
      dbg:text("--- %s %s %s%s%s ---", objsubtype, objtype, self.focus:getName(), acetext, boomtext)
      dbg:indent()
      if owner ~= nil then
        dbg:text("Owner: %s", owner:getName())
        dbg:indent()
          dbg:text("Credits: %d", owner:getCredits())
        dbg:undent()
      else
        dbg:text("Owner: [None]")
      end
      if not self.focus:isDestroyed() then
        if self.focus:hasHealth()then
          dbg:text("Health: %d%%", self.focus:getHealthPercent())
        end
        if string.match(objtype, "Station") and self.focus:hasDockable() then
          local docked = self.focus:getDocked()
          if docked and #docked > 0 then
            table.sort(docked, function (a, b) return a:getName() < b:getName() end)
            dbg:indent()
            dbg:text("Docked here:")
            dbg:indent()
              for _, v in ipairs(docked) do
                dbg:text("%s", v:getName())
              end
            dbg:undent()
            dbg:undent()
          end
        end
      end
      objval = self.focus:getRadius()
      if string.match(objtype, "Planet") then
        objval = objval * 9 -- planets need to be a certain radius for the game currently, so fake their reported radius for printing
      end
      objemit = "Radius: %d m"
      if objval > 120000000000 then
        objval = objval / 149598000000
        objemit = "Radius: %0.1f AU"
      elseif objval > 600000000 then
        objval = objval / 695700000
        objemit = "Radius: %0.1f Sr"
      elseif objval > 1000 then
        objval = objval / 1000
        objemit = "Radius: %0.1f km"
      end
      dbg:text(objemit, objval)
      objval = self.focus:getMass()
      objemit = "Mass: %0.0f kg"
      if objval > 1.0e28 then
        objval = objval / 1.99e30
        objemit = "Mass: %0.2f Sm"
      elseif objval > 5e23 then
        objval = objval / 5.97e24
        objemit = "Mass: %0.2f Em"
      elseif objval > 1000000 then
        objval = objval / 1000000
        objemit = "Mass: %0.1f mt"
      elseif objval > 907.18474 then
        objval = objval / 907.18474
        objemit = "Mass: %0.1f kt"
      end
      dbg:text(objemit, objval)
      if Config.game.currentShip then
        local posMe = Config.game.currentShip:getPos()
        local posIt = self.focus:getPos()
        objval = posMe:distance(posIt)
        -- TODO: Add check here to see if our ship is docked to the target, and set displayed range to 0 if so
        objemit = "Range: %0.0f m"
        if objval > 149598000 then
          objval = objval / 149598000
          objemit = "Range: %0.2f AU"
        elseif objval > 1000 then
          objval = objval / 1000
          objemit = "Range: %0.1f km"
        end
        dbg:text(objemit, objval)
      end
      self.focus:send(Event.Debug(dbg))
      dbg:undent()
    end
  end
end

function SystemMap:onInput (state)
  -- TODO: Connect to bindings (probably should be a new MapBindings.lua)
  -- NOTE: Keyboard pan and zoom previously used (e.g.) "kPanSpeed * state.dt"
  --       Removing that allows panning and zooming with keyboard to work when the game is Paused, but
  --       they may need to be reconnected to clock ticks if pan/zoom speeds are too dependent on local CPU
  Config.game.mapSystemZoom = Config.game.mapSystemZoom * exp(kZoomSpeed * Input.GetMouseScroll().y)
  Config.game.mapSystemPos.x = Config.game.mapSystemPos.x + (0.2 * kPanSpeed / Config.game.mapSystemZoom) * (
    Input.GetValue(Button.Keyboard.D) - Input.GetValue(Button.Keyboard.A))
  Config.game.mapSystemPos.y = Config.game.mapSystemPos.y + (0.2 * kPanSpeed / Config.game.mapSystemZoom) * (
    Input.GetValue(Button.Keyboard.S) - Input.GetValue(Button.Keyboard.W))
  Config.game.mapSystemZoom = Config.game.mapSystemZoom * exp(0.1 * kZoomSpeed * (
    Input.GetValue(Button.Keyboard.P) - Input.GetValue(Button.Keyboard.O)))
end

function SystemMap.Create (system)
  local self = setmetatable(UI.Window('System Map', false), SystemMap)
  self:setStretch(1, 1)
  self.system = system

  if Config.game.currentShip ~= nil then
    if Config.game.mapSystemPos == nil then
      -- Initialize system map starting position only if not already initialized
      Config.game.mapSystemPos = Config.game.currentShip:getPos()
    end
  else
    Config.game.mapSystemPos = Vec3f(0, 0, 0)
  end
  if Config.game.mapSystemZoom == nil then
    -- Initialize system map zoom level only if not already initialized
    Config.game.mapSystemZoom = 0.0001
  end
  return self
end

return SystemMap
