use crate::phases;


#[test]
fn iota() {
    let res = crate::parse("⍳200");
    assert!(res.is_ok());
    println!("{:?}", res);
}

#[test]
fn reduce() {
    let res = crate::parse("+/ 1 2 3 4");
    assert!(res.is_ok());
    println!("{:?}", res);
}

#[test]
fn reduce_iota() {
    let res = crate::parse("+/ ⍳200");
    assert!(res.is_ok());
    println!("{:?}", res);
    let output = res.unwrap();

    let phases = phases(&output);

    println!("{:?}", phases)
}