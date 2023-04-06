use markdown_compiler;
mod usage;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    usage::usage_utils::print_short_banner();

    match args.len() {
        1 => {
            println!("FILE NAME MISSING");
            usage::usage_utils::usage();
        },
        2 => {
            let filename = &args[1];
            let mut compiler_object = markdown_compiler::MarkdownCompiler::new(filename);
            compiler_object.compile();
        },
        _ => {            
            println!("INVALID ARGUMENTS");
            usage::usage_utils::usage();
        }
    }

}
