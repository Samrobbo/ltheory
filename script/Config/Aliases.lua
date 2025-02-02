-- Function aliases
insert = table.insert
remove = table.remove
format = string.format
join = table.concat

-- Slightly more indepth function aliases. Including Debug Functions for Trace.
function printf(...) print(format(...)) end

function trace() print(debug.traceback()) end

function traceFn()
    local info = debug.getinfo(2, 'nS')
    local file = info.short_src:match('[\\/]([^\\/%.]*)%.')
    printf('%s.%s', file, info.name)
end
