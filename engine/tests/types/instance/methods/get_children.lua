local foods = createInstanceTree {
    ClassName = "Part",
    Name = "Foo",
    Children = {
        Chicken = {
            ClassName = "Part",
            Children = {
                Roasted = {
                    ClassName = "Part"
                },
            },
        },
        Pork = {
            ClassName = "Part",
            Children = {
                Ham = {
                    ClassName = "Part",
                },
                Lechon = {
                    ClassName = "Part",
                }
            },
        },
        Finland = {
            ClassName = "Part",
        },
    },
}

local porkChildren = {
    foods.Pork.Ham,
    foods.Pork.Lechon,
}

local chickenChildren = {
    foods.Chicken.Roasted,
}

assert(deepEquals(foods.Pork:GetChildren(), porkChildren))
assert(deepEquals(foods.Chicken:GetChildren(), chickenChildren))
assert(deepEquals(foods.Finland:GetChildren(), {}))

assert(deepEquals(foods:GetChildren(), {
    foods.Chicken,
    foods.Pork,
    foods.Finland,
}))

assert(not deepEquals(foods:GetChildren(), {
    foods.Chicken,
    foods.Pork,
    foods.Finland,
    foods.Chicken.Roasted,
    foods.Pork.Ham,
    foods.Pork.Lechon,
}))
