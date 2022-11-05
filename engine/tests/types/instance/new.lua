-- creatable instances
local part = Instance.new("Part")
assert_eq(part.ClassName, "Part")

-- non-creatable instances
do
    local success = pcall(function()
        ---@diagnostic disable-next-line: invalid-class-name
        Instance.new("Foo")
    end)
    assert(not success)
end
