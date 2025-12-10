use rust_cli_app::SecretDial;

#[test]
fn test_get_initial_number() {
    let dial = SecretDial::new(100, 42);
    assert_eq!(dial.get_current_number(), 42);
}

#[test]
fn test_rotate_left_zero() {
    let mut dial = SecretDial::new(100, 50);
    dial.rotate("L0".to_string());

    assert_eq!(dial.get_current_number(), 50);
}

#[test]
fn test_rotate_r8_from_11() {
    let mut dial = SecretDial::new(100, 11);
    dial.rotate("R8".to_string());

    assert_eq!(dial.get_current_number(), 19);
}

#[test]
fn test_rotate_l19_from_19() {
    let mut dial = SecretDial::new(100, 19);
    dial.rotate("L19".to_string());

    assert_eq!(dial.get_current_number(), 0);
}

#[test]
fn test_rotate_l1_from_0() {
    let mut dial = SecretDial::new(100, 0);
    dial.rotate("L1".to_string());

    assert_eq!(dial.get_current_number(), 99);
}

#[test]
fn test_rotate_r1_from_99() {
    let mut dial = SecretDial::new(100, 99);
    dial.rotate("R1".to_string());

    assert_eq!(dial.get_current_number(), 0);
}

#[test]
fn test_rotate_l10_from_5() {
    let mut dial = SecretDial::new(100, 5);
    dial.rotate("L10".to_string());

    assert_eq!(dial.get_current_number(), 95);
}

#[test]
fn test_rotate_r5_from_95() {
    let mut dial = SecretDial::new(100, 95);
    dial.rotate("R5".to_string());

    assert_eq!(dial.get_current_number(), 0);
}

#[test]
fn test_rotate_many() {
    let mut dial = SecretDial::new(100, 11);
    dial.rotate_many(vec![
        "R8".to_string(),
        "L19".to_string(),
        "L1".to_string(),
        "R1".to_string(),
    ]);

    assert_eq!(dial.get_current_number(), 0);
}

#[test]
fn test_rotate_r1_from_3() {
    let mut dial = SecretDial::new(6, 3);
    dial.rotate("R1".to_string());

    assert_eq!(dial.get_current_number(), 4);
    assert_eq!(dial.get_zeroes(), 0);
}

#[test]
fn test_rotate_l1_from_3() {
    let mut dial = SecretDial::new(6, 3);
    dial.rotate("L1".to_string());

    assert_eq!(dial.get_current_number(), 2);
    assert_eq!(dial.get_zeroes(), 0);
}

#[test]
fn test_rotate_r3_from_3() {
    let mut dial = SecretDial::new(6, 3);
    dial.rotate("R3".to_string());

    assert_eq!(dial.get_current_number(), 0);
    assert_eq!(dial.get_zeroes(), 1);
}

#[test]
fn test_rotate_r9_from_3() {
    let mut dial = SecretDial::new(6, 3);
    dial.rotate("R9".to_string());

    assert_eq!(dial.get_current_number(), 0);
    assert_eq!(dial.get_zeroes(), 2);
}

#[test]
fn test_rotate_r1000_from_50() {
    let mut dial = SecretDial::new(100, 50);
    dial.rotate("R1000".to_string());

    assert_eq!(dial.get_current_number(), 50);
    assert_eq!(dial.get_zeroes(), 10);
}

#[test]
fn test_rotate_l100_from_50() {
    let mut dial = SecretDial::new(100, 50);
    dial.rotate("L100".to_string());

    assert_eq!(dial.get_current_number(), 50);
    assert_eq!(dial.get_zeroes(), 1);
}

#[test]
fn test_rotate_l1000_from_50() {
    let mut dial = SecretDial::new(100, 50);
    dial.rotate("L1000".to_string());

    assert_eq!(dial.get_current_number(), 50);
    assert_eq!(dial.get_zeroes(), 10);
}

#[test]
fn test_rotate_l9_from_3() {
    let mut dial = SecretDial::new(6, 3);
    dial.rotate("L9".to_string());

    assert_eq!(dial.get_current_number(), 0);
    assert_eq!(dial.get_zeroes(), 2);
}

#[test]
fn test_size_6_start_3_r1_zeroes_count_no_zeroes() {
    const SECRET_DOCUMENT: &[&str] = &[
        "R1",
    ];

    let mut dial = SecretDial::new(6, 3);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());

    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();

    assert_eq!(current, 4);
    assert_eq!(zeroes, 0);
}

#[test]
fn test_zeroes_count() {
    const SECRET_DOCUMENT: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    let mut dial = SecretDial::new(100, 50);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());

    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();

    assert_eq!(current, 32);
    assert_eq!(zeroes, 6);
}

#[test]
fn test_size_100_start_50_l68_zeroes_count_1() {
    const SECRET_DOCUMENT: &[&str] = &[
        "L68"
    ];
    
    let mut dial = SecretDial::new(100, 50);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());
    
    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();
    
    assert_eq!(current, 82);
    assert_eq!(zeroes, 1);
}

#[test]
fn test_size_100_start_82_l30_zeroes_count_0() {
    const SECRET_DOCUMENT: &[&str] = &[
        "L30"
    ];

    let mut dial = SecretDial::new(100, 82);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());

    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();

    assert_eq!(current, 52);
    assert_eq!(zeroes, 0);
}

#[test]
fn test_size_100_start_52_r48_zeroes_count_1() {
    const SECRET_DOCUMENT: &[&str] = &[
        "R48"
    ];

    let mut dial = SecretDial::new(100, 52);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());

    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();

    assert_eq!(current, 0);
    assert_eq!(zeroes, 1);
}

#[test]
fn test_size_100_start_0_l5_zeroes_count_0() {
    const SECRET_DOCUMENT: &[&str] = &[
        "L5"
    ];

    let mut dial = SecretDial::new(100, 0);
    dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());

    let current = dial.get_current_number();
    let zeroes = dial.get_zeroes();

    assert_eq!(current, 95);
    assert_eq!(zeroes, 0);
}




// #[test]
// fn test_size_6_start_3_r5_zeroes_count_1() {
//     const SECRET_DOCUMENT: &[&str] = &[
//         "R5"
//     ];
    
//     let mut dial = SecretDial::new(6, 3);
//     dial.rotate_many(SECRET_DOCUMENT.iter().map(|s| s.to_string()).collect());
    
//     let current = dial.get_current_number();
//     let zeroes = dial.get_zeroes();
    
//     assert_eq!(current, 2);
//     assert_eq!(zeroes, 1);
// }

