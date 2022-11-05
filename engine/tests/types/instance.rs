use crate::prelude::*;

test_code!(new, "instance/new.lua");
test_code!(__index, "instance/__index.lua");
test_code!(__newindex, "instance/__newindex.lua");

mod classes {
    use super::*;

    test_code!(part, "instance/classes/part.lua");
}

mod methods {
    use super::*;

    test_code!(find_first_child, "instance/methods/find_first_child.lua");
    test_code!(get_children, "instance/methods/get_children.lua");
    test_code!(get_descendants, "instance/methods/get_descendants.lua");
}
