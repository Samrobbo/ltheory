-- Resource --------------------------------------------------------------------
local ffi = require('ffi')
local libphx = require('ffi.libphx').lib
local Resource

do -- C Definitions
    ffi.cdef [[
    bool   Resource_Exists    (ResourceType, cstr name);
    cstr   Resource_GetPath   (ResourceType, cstr name);
    Bytes* Resource_LoadBytes (ResourceType, cstr name);
    cstr   Resource_LoadCstr  (ResourceType, cstr name);
  ]]
end

do -- Global Symbol Table
    Resource = {
        Exists    = libphx.Resource_Exists,
        GetPath   = libphx.Resource_GetPath,
        LoadBytes = libphx.Resource_LoadBytes,
        LoadCstr  = libphx.Resource_LoadCstr,
    }

    if onDef_Resource then onDef_Resource(Resource, mt) end
    Resource = setmetatable(Resource, mt)
end

return Resource
