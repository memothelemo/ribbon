-- Property check
local myInstance = Instance.new("Part")
assert_eq(myInstance.Name, "Part")

-- Children check
local foo = Instance.new("Part", myInstance)
foo.Name = "Bee"

local bee = myInstance.Bee
assert_eq(foo, bee)

-- Extra stuff
assert_eq(myInstance.Position, Vector3.new())
