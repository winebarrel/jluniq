use super::uniq;
use super::Opts;
use indoc::indoc;
use std::str;

static NDJSON: &str = indoc! {r#"
    {"id":1,"sub_id":11,"val":"abc"}
    {"id":2,"sub_id":21,"val":"ghi"}
    {"id":2,"sub_id":22,"val":"ghi"}
    {"id":3,"sub_id":31,"val":"jkl"}
    {"id":4,"sub_id":41,"val":"mno"}
    {"id":5,"sub_id":51,"val":"pqr"}
    {"id":5,"sub_id":52,"val":"slu"}
    {"id":6,"sub_id":61,"val":"vwz"}
"#};

#[test]
fn test_uniq1() {
    let reader = NDJSON.as_bytes();
    // let mut f = tempfile::tempfile().unwrap();
    // write!(f, "{}", NDJSON).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    uniq(
        reader,
        "id",
        fout,
        Opts {
            group: false,
            count: false,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            {"id":1,"sub_id":11,"val":"abc"}
            {"id":2,"sub_id":21,"val":"ghi"}
            {"id":3,"sub_id":31,"val":"jkl"}
            {"id":4,"sub_id":41,"val":"mno"}
            {"id":5,"sub_id":51,"val":"pqr"}
            {"id":6,"sub_id":61,"val":"vwz"}
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_uniq2() {
    let reader = NDJSON.as_bytes();
    // let mut f = tempfile::tempfile().unwrap();
    // write!(f, "{}", NDJSON).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    uniq(
        reader,
        "val",
        fout,
        Opts {
            group: false,
            count: false,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            {"id":1,"sub_id":11,"val":"abc"}
            {"id":2,"sub_id":21,"val":"ghi"}
            {"id":3,"sub_id":31,"val":"jkl"}
            {"id":4,"sub_id":41,"val":"mno"}
            {"id":5,"sub_id":51,"val":"pqr"}
            {"id":5,"sub_id":52,"val":"slu"}
            {"id":6,"sub_id":61,"val":"vwz"}
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_uniq_with_grouping() {
    let reader = NDJSON.as_bytes();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    uniq(
        reader,
        "id",
        fout,
        Opts {
            group: true,
            count: false,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [1,{"id":1,"sub_id":11,"val":"abc"}]
            [2,{"id":2,"sub_id":21,"val":"ghi"},{"id":2,"sub_id":22,"val":"ghi"}]
            [3,{"id":3,"sub_id":31,"val":"jkl"}]
            [4,{"id":4,"sub_id":41,"val":"mno"}]
            [5,{"id":5,"sub_id":51,"val":"pqr"},{"id":5,"sub_id":52,"val":"slu"}]
            [6,{"id":6,"sub_id":61,"val":"vwz"}]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_uniq_with_counting_grouping() {
    let reader = NDJSON.as_bytes();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    uniq(
        reader,
        "id",
        fout,
        Opts {
            group: true,
            count: true,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [1,1,{"id":1,"sub_id":11,"val":"abc"}]
            [2,2,{"id":2,"sub_id":21,"val":"ghi"},{"id":2,"sub_id":22,"val":"ghi"}]
            [3,1,{"id":3,"sub_id":31,"val":"jkl"}]
            [4,1,{"id":4,"sub_id":41,"val":"mno"}]
            [5,2,{"id":5,"sub_id":51,"val":"pqr"},{"id":5,"sub_id":52,"val":"slu"}]
            [6,1,{"id":6,"sub_id":61,"val":"vwz"}]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_uniq_with_counting() {
    let reader = NDJSON.as_bytes();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    uniq(
        reader,
        "id",
        fout,
        Opts {
            group: false,
            count: true,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [1,1]
            [2,2]
            [3,1]
            [4,1]
            [5,2]
            [6,1]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_join_without_key() {
    let reader = NDJSON.as_bytes();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    let r = uniq(
        reader,
        "noid",
        fout,
        Opts {
            group: false,
            count: false,
        },
    );

    assert_eq!(
        r.err().unwrap().to_string(),
        r#"Key 'noid' does not exist: {"id":1,"sub_id":11,"val":"abc"}"#
    );
}

#[test]
fn test_join_no_obj() {
    let reader = concat!(r#"[{"id":1}]"#, "\n", r#"[{"id":2}]"#).as_bytes();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    let r = uniq(
        reader,
        "noid",
        fout,
        Opts {
            group: false,
            count: false,
        },
    );

    assert_eq!(
        r.err().unwrap().to_string(),
        r#"JSON in row is not Object type: [{"id":1}]"#
    );
}
