return function(base, index, value)
    base[index] = value
    assert_eq(base[index], value)
end