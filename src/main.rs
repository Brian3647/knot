use std::fmt::Display;

use colored::Colorize;

fn main() {
	let mut args = std::env::args().skip(1);

	let first = args.next().unwrap_or_else(|| quit_with_help(1));

	// `--help` and `-help` aren't technically correct following the usage notes, but might
	// be what new users try first, so it's useful to include them.
	if first == "help" || first == "--help" || first == "-help" {
		quit_with_help(0);
	}

	match first.as_str() {
		"new" => todo!(),
		"remove" => todo!(),
		"change" => todo!(),
		"copy" => todo!(),
		_ => quit(1, format!("used knot: command `{first}` invalid")),
	}
}

fn quit(code: i32, done: impl Display) -> ! {
	let status = if code == 0 {
		"successfully".green()
	} else {
		"unsuccessfully".red()
	};

	println!("[{status} {done}]");
	std::process::exit(code);
}

fn quit_with_help(code: i32) -> ! {
	let knot = "knot".magenta();
	println!("{} v{}", knot, env!("CARGO_PKG_VERSION"));
	println!("{} knot <command> [args[]]", "usage:".bold(),);
	println!("{}", "where:".underline());

	println!(
		"{}: either `help`, `new`, `remove`, `change` or `copy`",
		"command".bold()
	);

	println!("\t{}: displays this message", "help".bold());
	println!("\t{}: creates a new knot", "new".bold());
	println!("\t{}: deletes an existing knot", "remove".bold());
	println!("\t{}: repaths an existing knot", "change".bold());

	println!(
		"\t{}: copies the dependencies of one project to another",
		"copy".bold()
	);

	println!("{}:", "examples".bold());
	println!("\t{}", "> knot help".black());
	println!(
		"\t{}",
		"> knot new /path/to/project /path/to/dep1 ...".black()
	);

	println!(
		"\t{}",
		"> knot remove /path/to/project /path/to/dep1".black()
	);

	println!("\t{}", "> knot remove /path/to/project *".black());
	println!(
		"\t{}",
		"> knot change /path/to/dep1 /new/path/to/dep1".black()
	);

	println!(
		"\t{}",
		"> knot copy /path/to/project1 /path/to/project2".black()
	);

	quit(
		code,
		if code == 0 {
			"displayed help message".into()
		} else {
			format!("used {knot}")
		},
	)
}
