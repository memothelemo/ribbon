-- Addition
do
    local _1 = Vector3.new()
    local _2 = Vector3.new()
    assert_eq(_1 + _2, Vector3.new())
end

do
    local _1 = Vector3.new(1, 0, 0)
    local _2 = Vector3.new(2, 3, 4)
    assert_eq(_1 + _2, Vector3.new(3, 3, 4))
end

-- Subtraction
do
    local _1 = Vector3.new()
    local _2 = Vector3.new()
    assert_eq(_1 - _2, Vector3.new())
end
do
    local _1 = Vector3.new(1, 0, 0)
    local _2 = Vector3.new(2, 3, 4)
    assert_eq(_1 - _2, Vector3.new(-1, -3, -4))
end

-- Multiplication
do
    local _1 = Vector3.new()
    local _2 = Vector3.new()
    assert_eq(_1 * _2, Vector3.new())
end
do
    local _1 = Vector3.new(1, 0, 0)
    local _2 = Vector3.new(2, 3, 4)
    assert_eq(_1 * _2, Vector3.new(2, 0, 0))
end
do
    local _1 = Vector3.new()
    local _2 = Vector3.new()
    assert_eq(_1 * 3, Vector3.new())
end
do
    local _1 = Vector3.new(1, 2, 3)
    assert_eq(_1 * 5, Vector3.new(5, 10, 15))
end

-- Division
do
    -- otherwise it will result as NaN value
    -- if it is divided by 0
    local _1 = Vector3.new(1, 1, 1)
    local _2 = Vector3.new(1, 1, 1)
    assert_eq(_1 / _2, Vector3.new(1, 1, 1))
end
do
    local _1 = Vector3.new(1, 0, 0)
    local _2 = Vector3.new(2, 3, 4)
    assert_eq(_1 / _2, Vector3.new(0.5, 0, 0))
end
do
    local _1 = Vector3.new(10, 8, 6)
    assert_eq(_1 / 2, Vector3.new(5, 4, 3))
end