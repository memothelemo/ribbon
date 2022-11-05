local myInstance = Instance.new("Part")

-- base instance properties
myInstance.Name = "Foo"

-- extended
myInstance.Position = Vector3.new(2, 3, 4)
assert_eq(myInstance.Position, Vector3.new(2, 3, 4))
