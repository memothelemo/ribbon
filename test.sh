cargo build
valgrind --show-leak-kinds=all --leak-check=full ./target/debug/ribbon