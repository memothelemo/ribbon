do
    local vector = Vector3.new(1, 1, 1)
    assert_eq(vector, Vector3.new(1, 1, 1))
end
do
    local vector = Vector3.new(1, 1, 2)
    assert_eq(vector, Vector3.new(1, 1, 2))
end
do
    local vector = Vector3.new(1, 2, 1)
    assert_eq(vector, Vector3.new(1, 2, 1))
end
do
    local vector = Vector3.new(2, 1, 1)
    assert_eq(vector, Vector3.new(2, 1, 1))
end