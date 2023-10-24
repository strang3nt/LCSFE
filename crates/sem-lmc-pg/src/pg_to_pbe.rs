use crate::pg::{Player, PG};
use sem_lmc_algorithm::ast::fixpoint_system::{ExpFixEq, FixEq, FixType};

pub fn pg_to_pbe(pg: &PG, p: Player) -> Vec<FixEq> {
    pg.0.iter().fold(vec![], |mut acc, (n, adj_list)| {
        let x = format!("x_{}", n.id);

        let fix_ty = if n.parity % 2 == 0 { FixType::Max } else { FixType::Min };

        let mut adj_list_iter = adj_list.iter();
        let first = ExpFixEq::Id(format!("x_{}", adj_list_iter.next().unwrap()));
        let right_hand = adj_list_iter.fold(first, |acc, elem| {
            let x_i = ExpFixEq::Id(format!("x_{}", elem));
            match acc {
                id @ ExpFixEq::Id(_) if n.owner == p => ExpFixEq::Or(Box::new(id), Box::new(x_i)),
                id @ ExpFixEq::Id(_) => ExpFixEq::And(Box::new(id), Box::new(x_i)),
                or @ ExpFixEq::Or(_, _) =>
                    ExpFixEq::Or(Box::new(or), Box::new(x_i)),
                and @ ExpFixEq::And(_, _) =>
                    ExpFixEq::And(Box::new(and), Box::new(x_i)),
                _ => panic!("Encountered a type {:#?} while building a fixpoint equation", acc),
            }
        });
        acc.push(FixEq { var: x, fix_ty: fix_ty, exp: right_hand });
        acc

    })
}
