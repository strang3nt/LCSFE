use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

// aut_header        ::=  'des (' first_state ',' nr_of_transitions ',' nr_of_states ')'
// first_state       ::=  number
// nr_of_transitions ::=  number
// nr_of_states      ::=  number
// aut_edge    ::=  '(' start_state ',' label ',' end_state ')'
// start_state ::=  number
// label       ::=  '"' string '"'
// end_state   ::=  number
pub struct Lts {
    pub first_state: u32,
    pub labels: Vec<String>,
    pub adj_list: HashMap<u32, Vec<(usize, u32)>>,
}

pub fn ald_parser(src: &mut BufReader<File>) -> Result<Lts, Error> {
    let header: String = src.lines().next().unwrap()?;
    assert!(header.starts_with("des (") && header.ends_with(')'));
    let header: Vec<u32> = header
        .trim_start_matches("des (")
        .trim_end_matches(')')
        .split(',')
        .map(|x| x.parse::<u32>().expect("Expected natural number"))
        .collect();
    let first_state = header[0];
    let nr_of_states: usize = header[2].try_into().unwrap();
    let mut labels: Vec<String> = vec![];
    let adj_list = src.lines().try_fold(HashMap::with_capacity(nr_of_states), |mut acc, elem| {
        match elem.map(|x| {
            let x_trim = x.trim_start_matches('(').trim_end_matches(')');
            let start = x_trim.splitn(2, ',').collect::<Vec<&str>>();
            let label: Vec<&str> = start[1].trim_start_matches('"').splitn(2, '"').collect();
            let end = label[1].trim_start_matches(',');

            vec![start[0].to_string(), label[0].to_string(), end.to_string()]
        }) {
            Ok(edge) => {
                let start_node: u32 = edge[0].parse().unwrap();
                let label: String = edge[1].parse().unwrap();
                let end_node: u32 = edge[2].parse().unwrap();
                let position: usize = if let Some(i) = labels.iter().position(|x| x == &label) {
                    i
                } else {
                    labels.push(label.to_owned());
                    labels.len() - 1
                };
                if acc.get(&start_node).is_some() {
                    acc.get_mut(&start_node);
                    Ok(acc)
                } else {
                    acc.insert(start_node, vec![(position, end_node)]);
                    Ok(acc)
                }
            }
            Err(e) => Err(e),
        }
    });

    assert!(adj_list.as_ref().unwrap().len() == nr_of_states);
    Ok(Lts { first_state, labels, adj_list: adj_list? })
}
