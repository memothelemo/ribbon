cargo build && valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./target/debug/ribbon