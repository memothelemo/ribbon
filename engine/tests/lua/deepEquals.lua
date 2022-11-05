local function deepEqual(a: any, b: any)
    if typeof(a) ~= typeof(b) then
        return false
    end

    if typeof(a) == "table" then
        local visitedKeys = {}

        for key, value in pairs(a) do
            visitedKeys[key] = true

            local success = deepEqual(value, b[key])
            if not success then
                return false
            end
        end

        for key, value in pairs(b) do
            if not visitedKeys[key] then
                local success = deepEqual(value, a[key])
                if not success then
                    return false
                end
            end
        end

        return true
    end

    if a == b then
        return true
    end

    return false
end

return deepEqual