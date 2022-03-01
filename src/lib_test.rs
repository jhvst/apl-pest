
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

    let phases = crate::phases(&output);

    println!("{:?}", phases)
}

#[test]
fn phased_execute() {

    let input = r#"
        {"phases":[{"idx":0,"input":"shape (Reduce (SomeVect 4))","output":"SomeScalar : Shape FZ (MkDim 1 1)","size":1}]}
    "#;

    let phases: crate::Phases = serde_json::from_str(input).unwrap();

    let results = crate::vk_compute(phases, "+/ 1 2 3 4");

    println!("{:?}", results)
}

#[test]
fn demo_execute() {

    let input = r#"
        {"phases":[{"idx":0,"input":"shape (IndexGenerator 200)","output":"SomeVect 201 : Shape (FS FZ) (MkDim 1 201)","size":201},{"idx":1,"input":"shape (Reduce (shape (IndexGenerator 200)))","output":"SomeScalar : Shape FZ (MkDim 1 1)","size":1}]}
    "#;

    let phases: crate::Phases = serde_json::from_str(input).unwrap();

    let results = crate::vk_compute(phases, "+/ ⍳200");

    println!("{:?}", results)
}

