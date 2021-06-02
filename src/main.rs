// use rusqlite::{params, Connection, Result};
use sqlite;
#[derive(Debug)]
struct Family {
    famid: String,
    member_name: String,
    // data: Option<Vec<u8>>,
}

fn get_family_data_from_db(_famid: u32) -> Vec<(String, String)>{
    let connection = sqlite::open("sqlite.db").unwrap();
    let mut result = Vec::new();
    let statement = format!(r#"
    select hof.hof,  m.members from (select famid, group_concat(member_name, '{delimiter}') as members
    from members group by famid) as m left join
    (select famid, member_name as hof from members where rltshp = "Self") as hof
    on m.famid = hof.famid;
    "#, delimiter=", ");

    connection.iterate(statement, |row| { // format!("SELECT famid, member_name FROM members WHERE famid > {}", famid
        let family = row.iter().map(
            |x| {
                match x.1 {
                    None => " ".to_string(),
                    Some(v) => v.to_string()
                }
            }
        ).collect::<Vec<_>>(); // x.0 for selectinn first item in tuple inside list   | ).collect::<Vec<_>>();
        println!("{:?}", family.get(0).unwrap());
        result.push((family.get(0).unwrap().to_string(), family.get(1).unwrap().to_string()));
        true
    })
    .unwrap();
    // println!("{:?}", result);
    result
}

fn main() {
    get_family_data_from_db(10);
}

