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

assert(foods:FindFirstChild("Lechon") == nil)
assert(foods:FindFirstChild("Ham") == nil)

assert(foods:FindFirstChild("Chicken") ~= nil)
assert(foods:FindFirstChild("Pork") ~= nil)

assert_eq(foods:FindFirstChild("Chicken"), foods.Chicken)
assert_eq(foods:FindFirstChild("Pork"), foods.Pork)
assert_eq(foods:FindFirstChild("Pork"):FindFirstChild("Ham"), foods.Pork.Ham)

assert(foods:FindFirstChild("Lechon", true) ~= nil)
assert(foods:FindFirstChild("Ham", true) ~= nil)

assert(foods:FindFirstChild("Chicken", true) ~= nil)
assert(foods:FindFirstChild("Pork", true) ~= nil)

assert_eq(foods:FindFirstChild("Lechon", true), foods.Pork.Lechon)
assert_eq(foods:FindFirstChild("Roasted", true), foods.Chicken.Roasted)
assert_eq(foods:FindFirstChild("Pork"), foods.Pork)
