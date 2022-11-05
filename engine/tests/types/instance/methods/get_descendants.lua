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

assert(deepEquals(foods.Pork:GetDescendants(), porkChildren))
assert(deepEquals(foods.Chicken:GetDescendants(), chickenChildren))
assert(deepEquals(foods.Finland:GetDescendants(), {}))

assert((#foods:GetDescendants()) == 6)
assert(deepEquals(foods:GetDescendants(), {
    foods.Chicken,
    foods.Chicken.Roasted,
    foods.Pork,
    foods.Pork.Ham,
    foods.Pork.Lechon,
    foods.Finland,
}))
