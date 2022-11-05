return function(tree)
    local function createInner(tree)
        local instance = Instance.new(tree.ClassName)
        for index, value in pairs(tree) do
            if index ~= "ClassName" and index ~= "Children" then
                instance[index] = value
            end
        end
        for name, inner in pairs(tree.Children or {}) do
            inner.Name = name
            inner.Parent = instance
            createInner(inner)
        end
        return instance
    end
    return createInner(tree)
end