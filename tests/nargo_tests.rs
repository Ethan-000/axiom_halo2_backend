use std::{
    fs,
    io::Result,
    path::PathBuf,
    process::{Command, Output},
};

#[allow(dead_code)]
fn configure_test_dirs() -> Vec<PathBuf> {
    let test_dirs_names = vec![
        // "1_mul",
        // "2_div",
        // "3_add",
        // "4_sub",
        // "5_over",
        // "6_array",
        // "7_function",
        // "8_bit_and",
        "9_public_io",
    ];
    test_dirs_names
        .into_iter()
        .map(test_program_dir_path)
        .collect()
}

fn nargo_cmd() -> std::process::Command {
    Command::new("nargo")
}

fn nargo_execute(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("execute")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_test(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("test")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_check(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("check")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_gates(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("gates")
        .spawn()
        .unwrap()
        .wait_with_output()
}

pub fn nargo_codegen_verifier(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("codegen-verifier")
        .spawn()
        .unwrap()
        .wait_with_output()
}

pub fn nargo_compile(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("compile")
        .arg("my_test_circuit")
        .spawn()
        .unwrap()
        .wait_with_output()
}

pub fn nargo_prove(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("prove")
        .arg("my_test_proof")
        .arg("my_test_circuit")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_verify(test_program_dir: &PathBuf) -> Result<Output> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("verify")
        .arg("my_test_proof")
        .arg("my_test_circuit")
        .spawn()
        .unwrap()
        .wait_with_output()
}

pub fn test_program_dir_path(dir_name: &str) -> PathBuf {
    fs::canonicalize(PathBuf::from(format!("./tests/test_programs/{dir_name}"))).unwrap()
}

fn assert_nargo_cmd_works(cmd_name: &str, test_test_program_dir: &PathBuf) {
    let cmd_output = match cmd_name {
        "check" => nargo_check(test_test_program_dir),
        "codegen-verifier" => nargo_codegen_verifier(test_test_program_dir),
        "compile" => nargo_compile(test_test_program_dir),
        "new" => panic!("This cmd doesn't depend on the backend"),
        "execute" => nargo_execute(test_test_program_dir),
        "prove" => nargo_prove(test_test_program_dir),
        "verify" => nargo_verify(test_test_program_dir),
        "test" => nargo_test(test_test_program_dir),
        "gates" => nargo_gates(test_test_program_dir),
        e => panic!("{e} is not a valid nargo cmd"),
    }
    .unwrap();

    assert!(
        cmd_output.status.success(),
        "stderr(nargo {cmd_name}) in {}: {}",
        test_test_program_dir.display(),
        String::from_utf8(cmd_output.stderr).unwrap()
    );
}

pub fn install_nargo(backend: &'static str) {
    // Clone noir into repo
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/Ethan-000/noir")
        .arg("--branch")
        .arg("add_halo2_backend")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    format!("\nInstalling {backend}. This may take a few moments.",);
    // Install specified backend into noir
    Command::new("cargo")
        .current_dir(fs::canonicalize("./noir/crates/nargo_cli").unwrap())
        .arg("install")
        .arg("--path")
        .arg(".")
        .arg("--locked")
        .arg("--features")
        .arg(backend)
        .arg("--no-default-features")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn run_nargo_tests(test_program: PathBuf) {
    assert_nargo_cmd_works("check", &test_program);
    assert_nargo_cmd_works("codegen-verifier", &test_program);
    assert_nargo_cmd_works("compile", &test_program);
    assert_nargo_cmd_works("execute", &test_program);
    assert_nargo_cmd_works("prove", &test_program);
    assert_nargo_cmd_works("verify", &test_program);
    assert_nargo_cmd_works("test", &test_program);
    assert_nargo_cmd_works("gates", &test_program);
}

#[cfg(feature = "axiom_halo2")]
#[test]
fn test_axiom_backend() {
    let test_program_dirs = configure_test_dirs();
    // Pass in Axiom Halo2 Backend as argument
    install_nargo("axiom_halo2_backend");
    for test_program in test_program_dirs {
        run_nargo_tests(test_program);
    }
}

#[cfg(feature = "pse_halo2")]
#[test]
fn test_pse_backend() {
    let test_program_dirs = configure_test_dirs();
    // Pass in PSE Halo2 Backend as argument
    install_nargo("pse_halo2_backend");
    for test_program in test_program_dirs {
        run_nargo_tests(test_program);
    }
}
