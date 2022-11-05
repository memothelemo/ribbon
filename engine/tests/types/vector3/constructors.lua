do
    local vector = Vector3.new()
    assert_eq(vector, Vector3.new())
end
do
    local vector = Vector3.one
    assert_eq(vector, Vector3.new(1, 1, 1))
end
do
    local vector = Vector3.xAxis
    assert_eq(vector, Vector3.new(1, 0, 0))
end
do
    local vector = Vector3.yAxis
    assert_eq(vector, Vector3.new(0, 1, 0))
end
do
    local vector = Vector3.zAxis
    assert_eq(vector, Vector3.new(0, 0, 1))
end