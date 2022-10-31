local thing = Instance.new("Part")
thing.Name = "Foo"

for i = 1, 100_000 do
    local _ = Instance.new("Part", thing)
    _.Name = tostring(i)
end
