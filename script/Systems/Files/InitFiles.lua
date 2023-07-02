local InitFiles = {}

local writeDenylist = {
    categories = {
        "world"
    },
    variables = {
        "humanPlayer"
    },
    variableSubStrings = {
        "current",
        "lastCamera",
        "playerMoving",
        "weaponGroup",
        "autonavTimestamp",
        "mapSystemZoom"
    }
}

-- Only write strings, booleans and numbers. 
local writeAllowlist = {
    types = {
        "string",
        "boolean",
        "number",
        "table" -- Tables are iterated over to all leaf nodes.
    }
}

function InitFiles:readUserInits()
    -- Reads user initialization values from file
    -- TODO: Encase io.xxx functions in local wrappers for security/safety
    local filename = Config.userInitFilename
    local filepath = Config.paths.files
    local configPath = filepath .. filename
    local openedFile = io.open(configPath, "r")

    if openedFile then
        local lines = {}

        -- Sets the input file for reading
        io.input(openedFile)

        -- Reads all lines from the file
        for line in openedFile.lines(openedFile) do
            lines[#lines + 1] = line
        end

        -- Closes the open file
        io.close(openedFile)

        -- Scan all lines and apply values to matched game values
        local stringToBoolean = { ["true"] = true, ["false"] = false }

        local function findCategory(line)
            if not string.match(line, "=") then
                return true
            end
            return false
        end

        local categories = {}

        for index, line in ipairs(lines) do
            -- Skip comments
            if string.sub(line, 1, 1) == "#" then
                goto skip
            end

            if findCategory(line) then
                local categoryName
                local subCategoryName
                categoryName = string.gsub(line, "%[", "")
                categoryName = string.gsub(categoryName, "%]", "")

                if string.match(categoryName, "%.") then
                    subCategoryName = string.gsub(categoryName, ".*%.", "")
                    categoryName = string.gsub(categoryName, "%..*", "")
                end

                local gameStateTable = GameState[categoryName]

                if subCategoryName then
                    gameStateTable = GameState[categoryName][subCategoryName]
                end

                if gameStateTable then
                    local categoryTable = {
                        name = categoryName,
                        gameState = gameStateTable,
                        index = index,
                        vars = {}
                    }
                    table.insert(categories, categoryTable)
                else
                    Log.Warning("Could not find game state for config category: " .. categoryName)
                end
            end
            ::skip::
        end

        local function findValuesForCategory(categoryTable)
            local function checkIfCursorStyle(val)
                for cursorStyle = 1, Enums.CursorStyleCount do
                    if string.match(string.lower(val), string.lower(Enums.CursorStyleNames[cursorStyle])) then
                        return cursorStyle
                    end
                end
                return nil
            end

            local function checkIfHudStyle(val)
                for hudStyle = 1, Enums.HudStyleCount do
                    if string.match(string.lower(val), string.lower(Enums.HudStyleNames[hudStyle])) then
                        return hudStyle
                    end
                end
                return nil
            end

            local function checkIfCameraMode(val)
                for cameraMode = 1, Enums.CameraModeCount do
                    if string.match(string.lower(val), string.lower(Enums.CameraModeNames[cameraMode])) then
                        return cameraMode
                    end
                end
                return nil
            end

            local function firstToLower(string)
                return (string:gsub("^%L", string.lower))
            end

            local function firstToUpper(string)
                return (string:gsub("^%l", string.upper))
            end

            local function setValue(var, val)
                local lower = firstToLower(var)
                local upper = firstToUpper(var)

                if categoryTable.gameState[lower] ~= nil then
                    categoryTable.gameState[lower] = val
                elseif categoryTable.gameState[upper] ~= nil then
                    categoryTable.gameState[upper] = val
                else
                    Log.Warning("Can't find key in gamestate cat %s for var: %s with value %s", categoryTable.name, var,
                        val)
                end
            end

            local iterator = tonumber(categoryTable.index) + 1
            local vars = {}
            local currentLine = lines[iterator]

            while currentLine and not string.match(currentLine, "%[") do
                -- skip comments
                if string.match(currentLine, "#") then
                    iterator = iterator + 1
                    currentLine = lines[iterator]
                    goto skipLine
                end

                -- parse vars
                local eIndex = string.find(currentLine, "=")
                --printf("Line %s: %s", iterator, currentLine)
                --printf("Current eIndex: %s", eIndex)
                local var = string.sub(currentLine, 1, eIndex - 1)
                local val = string.sub(currentLine, eIndex + 1)
                val = string.gsub(val, "^%s*(.-)%s*$", "%1")

                if val == "true" or val == "false" then
                    local bool = stringToBoolean[val]
                    setValue(var, bool)
                elseif tonumber(val) then
                    setValue(var, tonumber(val))
                elseif checkIfCursorStyle(val) then
                    local style = checkIfCursorStyle(val)
                    setValue(var, style)
                    val = tostring(style)
                elseif checkIfHudStyle(val) then
                    local style = checkIfHudStyle(val)
                    setValue(var, style)
                    val = tostring(style)
                elseif checkIfCameraMode(val) then
                    local mode = checkIfCameraMode(val)
                    setValue(var, mode)
                    val = tostring(mode)
                else
                    setValue(var, val)
                end

                iterator = iterator + 1
                currentLine = lines[iterator]
                --printf("Setting var to gamestate: %s with value: %s", var, val)
                ::skipLine::
            end
            return vars
        end

        for _, categoryTable in ipairs(categories) do
            categoryTable.vars = findValuesForCategory(categoryTable)
            -- do whatever with vars if needed
        end
        printf("Loaded configuration from %s", configPath)

        if GameState.debug.printConfig then
            print("---------- Configuration File ----------")
            for _, line in pairs(lines) do if not string.match(line, "#") then print(line) end end
            print("----------------------------------------")
        end
    end
end

function InitFiles:writeUserInits()
    -- Writes user initialization values to file
    -- TODO: Encase io.xxx functions in local wrappers for security/safety
    local filename = Config.userInitFilename
    local filepath = Config.paths.files
    local openedFile = io.open(filepath .. filename, "w")

    local cursorType = string.lower(Enums.CursorStyleNames[GameState.ui.cursorStyle])
    local hudType = string.lower(Enums.HudStyleNames[GameState.ui.hudStyle])
    local startupCameraMode = string.lower(Enums.CameraModeNames[GameState.player.currentCamera])

    -- Sets the input file for writing
    io.output(openedFile)

    local function shouldWrite(variable, value)
        -- Check if key is denylisted directly.
        for _,denyKey in pairs(writeDenylist.variables) do
            if variable == denyKey then
                return false
            end
        end
    
        -- Check if key contains denylisted substring.
        for _,prefix in pairs(writeDenylist.variableSubStrings) do
            if string.match(variable, prefix) then
                return false;
            end
        end
    
        -- Check if type(value) is allowlisted.
        for _,allowedType in pairs(writeAllowlist) do
            if type(value) == allowedType then
                return true
            end
        end
    end
    
    local function comment(commentTable)
        local commentString = ""
        -- Config Desc
        for _, headerLine in ipairs(commentTable) do
            commentString = commentString .. format("# %s\n", headerLine)
        end
        return commentString
    end
    
    local function options(optionTitle, optionTable, optionDesc)
        local optionString = string.lower(table.concat(optionTable, ", "))
        return comment {
            optionTitle .. "Options: <" .. optionString .. ">",
            optionDesc
        }
    end

    local function variableValueString(variableKey, value)
        
        local prefixLines = ""
        if variableKey == "cursorStyle" then
            value = cursorType
            prefixLines = options("cursorStyle", Enums.CursorStyleNames, "The game`s currently used cursor style.")
        elseif variableKey == "hudStyle" then
            value = hudType
            prefixLines = options("hudStyle", Enums.HudStyleNames, "The game`s currently used hud style.")
        elseif variableKey == "startupCamera" then
            value = startupCameraMode
            prefixLines = options("startupCamera", Enums.CameraModeNames, "The camera mode the game starts up with.")
        end
        -- printf("writing %s: %s", l_Variable, l_Value)
        return prefixLines .. format("%s=%s\n", tostring(variableKey), tostring(value))
    end
    
    local function categoryString(categoryKey, categoryTable)
        -- Return early if category is denylisted.
        for _,denyKey in pairs(writeDenylist.categories) do
            if categoryKey == denyKey then
                return ""
            end
        end
    
        local catFileString = format("[%s]\n", tostring(categoryKey))
        local subCatFileString = ""
    
        for variable, value in pairsByKeys(categoryTable) do
            -- printf("Category: %s | Variable: %s | Val: %s (type: %s)", categoryKey, l_Variable, l_Value, type(l_Value))
            if shouldWrite(variable, value) then
                if type(value) == "table" then
                    subCatFileString = subCatFileString .. categoryString(categoryKey .. "." .. variable, value)
                else
                    catFileString = catFileString .. variableValueString(variable, value)
                end
            end
        end
    
        return catFileString .. subCatFileString
    end
    
    local function pairsByKeys(t, f)
        local a = {}
        for n in pairs(t) do table.insert(a, n) end
        table.sort(a, f)
        local i = 0             -- iterator variable
        local iter = function() -- iterator function
            i = i + 1
            if a[i] == nil then
                return nil
            else
                return a[i], t[a[i]]
            end
        end
        return iter
    end

    io.write(comment {
        "Hello World! This is the Limit Theory Redux Configuration File",
        "Support the LTR project by discussing, contributing or silent participation:",
        "GitHub: " .. Config.orgInfo.repository,
        "Discord: " .. Config.orgInfo.discord
    })

    -- Clean up GameState table
    local categories = {}

    for l_Index, l_Value in pairs(GameState) do
        if type(l_Value) == "table" then
            categories[l_Index] = l_Value
        end
    end

    for l_Category, l_CategoryTable in pairsByKeys(categories) do
        io.write(categoryString(l_Category, l_CategoryTable))
    end
    -- Closes the open file
    io.close(openedFile)
end

return InitFiles
