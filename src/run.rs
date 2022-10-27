use std::process::Command;

use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

// Invoke the rust compiler on the path of the given exercise,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the exercise is test)
pub fn run(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => return test(exercise, verbose),
        Mode::Compile => return compile_and_run(exercise),
        Mode::Clippy => return compile_and_run(exercise),
    }
    // Ok(())
}

// Resets the exercise by stashing the changes.
pub fn reset(exercise: &Exercise) -> Result<(), ()> {
    let command = Command::new("git")
        .args(["stash", "--"])
        .arg(&exercise.path)
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

// Invoke the rust compiler on the path of the given exercise
// and run the ensuing binary.
// This is strictly for non-test binaries, so output is displayed
fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compilation of {} failed!, Compiler error message:\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("Running {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("output.stdout: {}", output.stdout);
            success!("Successfully ran {}", exercise);
            Ok(())
        }
        Err(output) => {
            println!("output.stdout: {}", output.stdout);
            println!("output.stderr: {}", output.stderr);

            warn!("Ran {} with errors", exercise);
            Err(())
        }
    }
}


#[cfg(test)]
mod test {
    use crate::exercise::temp_file;

    use super::*;
    use std::{path::{Path, PathBuf}, fs::File};

    #[test]
    fn test_clean() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            name: String::from("example"),
            // path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
            path: PathBuf::from("exercises/clippy/clippy1.rs"),
            mode: Mode::Clippy,
            hint: String::from(""),
        };
        // let compiled = exercise.compile().unwrap();
        let result = compile_and_run(&exercise);
        match result {
            Ok(result) => {
                println!("success!");
            },
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
        // drop(compiled);
    }

}
