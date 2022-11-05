use crate::prelude::*;

test_code!(constructors, "vector3/constructors.lua");
test_code!(compare, "vector3/compare.lua");
test_code!(new, "vector3/new.lua");

test_code!(__index, "vector3/__index.lua");
test_code!(__newindex, "vector3/__newindex.lua");
test_code!(__op, "vector3/__op.lua");
