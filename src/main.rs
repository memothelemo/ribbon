// fn init_logger() {
//     let env = env_logger::Env::default().default_filter_or("info,ribbon_engine=trace,ribbon=trace");

//     env_logger::builder()
//         .parse_env(env)
//         .format_module_path(false)
//         .format_timestamp(None)
//         .format_indent(Some(8))
//         .init();
// }

fn main() {
    // init_logger();

    ribbon_engine::main().map_err(|e| println!("{e}")).unwrap();
}
