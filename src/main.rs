use std::{
    fs, io,
    path::PathBuf,
    process::{Command, ExitCode},
};

mod resolver;

#[derive(Debug)]
struct Target {
    name: String,
    depends: Vec<String>,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    executable: String,
    args: Vec<String>,
}

fn needs_rebuild(target: &Target) -> io::Result<bool> {
    // Check for any missing given inputs
    for input in &target.inputs {
        if !input.try_exists()? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Input '{}' does not exist", input.display()),
            ));
        }
    }

    if !target.output.try_exists()? {
        return Ok(true);
    }

    let output_time = fs::metadata(&target.output)?.modified()?;

    for input in &target.inputs {
        let input_time = fs::metadata(input)?.modified()?;

        if input_time > output_time {
            return Ok(true);
        }
    }

    Ok(false)
}

fn build(target: &Target) -> io::Result<()> {
    let executable = &target.executable;

    println!("sprout: {} {}", executable, target.args.join(" "));

    let status = Command::new(&target.executable)
        .args(&target.args)
        .status()?;

    if !status.success() {
        return Err(io::Error::other("Compilation failed"));
    }

    Ok(())
}

fn main() -> ExitCode {
    // Hardcode values until I make a configuration language
    let target = Target {
        name: String::from("main")
        inputs: vec![PathBuf::from("src/main.c")],
        output: PathBuf::from("main.o"),
        executable: String::from("cc"),
        args: vec![
            "-c".into(),
            "src/main.c".into(),
            "-o".into(),
            "main.o".into(),
        ],
    };

    match needs_rebuild(&target) {
        Ok(true) => {
            if let Err(err) = build(&target) {
                eprintln!("sprout: {err}");
                return ExitCode::FAILURE;
            }
        }
        Ok(false) => println!("sprout: nothing to build"),
        Err(err) => {
            eprintln!("sprout: {err}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
