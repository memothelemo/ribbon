local _1 = Vector3.new(1, 2, 3)
local _2 = Vector3.new(2, 3, 4)
assert(_1 ~= _2)

local _3 = Vector3.new(1, 2, 3)
local _4 = Vector3.new(1, 3, 4)
assert(_3 ~= _4)

local _5 = Vector3.new()
local _6 = Vector3.new()
assert(_5 == _6)

local _7 = Vector3.new(5, 5, 5)
local _8 = Vector3.new(5, 5, 5)
assert(_7 == _8)
