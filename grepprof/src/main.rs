use std::{env, fs::File};

use minigrep::{argparse::MiniGrepArg, run_single_thread, GrepGroup, RunArg};

fn main_fn() {
    // parse arg
    let my_arg = match MiniGrepArg::new(env::args()) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Argument parse error!", v),
    };

    // create grep
    let my_re = GrepGroup::from_re_group(
        my_arg.expr,
        my_arg.extract_expr,
        my_arg.replace_expr,
        my_arg.replacer,
        my_arg.replace_times,
        my_arg.ignorecase,
        my_arg.color_flag,
    )
    .expect("GrepGroup build failed");

    // run arg
    let run_arg = RunArg {
        ahead_size: my_arg.ahead_size,
        behind_size: my_arg.behind_size,
        file_path_flag: my_arg.file_path_flag,
        line_num_flag: my_arg.line_num_flag,
    };

    run_single_thread(
        my_re,
        run_arg,
        &my_arg.file_path,
        my_arg.skip_hidden,
        my_arg.max_depth,
    )
    .unwrap();
}

fn main() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    main_fn();

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        let mut options = pprof::flamegraph::Options::default();
        options.image_width = Some(2500);
        report.flamegraph_with_options(file, &mut options).unwrap();
    };
}
