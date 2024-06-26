use extrasafe::SafetyContext;
use syscalls::Sysno;

#[test]
#[should_panic(expected = "NoRulesEnabled")]
/// Extrasafe fails by default if there are no rules applied
fn hello_void() {
    SafetyContext::new().apply_to_current_thread().unwrap();

    println!("Hello, World!");
}

#[test]
fn hello_world() {
    SafetyContext::new()
        .enable(Sysno::write)
        .unwrap()
        .apply_to_current_thread()
        .unwrap();

    println!("Hello, World!");
}
