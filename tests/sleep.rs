use extrasafe::builtins::{SystemIO, danger_zone::Threads};

#[test]
// assert is just unhelpful assert_eq message
#[should_panic(expected = "assertion")]
fn insomnia() {
    extrasafe::SafetyContext::new()
        .enable(SystemIO::nothing()
            .allow_stdout()
            .allow_stderr()).unwrap()
        .apply_to_current_thread().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn comfy() {
    extrasafe::SafetyContext::new()
        .enable(SystemIO::nothing()
            .allow_stdout()
            .allow_stderr()).unwrap()
        .enable(Threads::nothing()
            .allow_sleep().yes_really()).unwrap()
        .apply_to_current_thread().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1));
}
