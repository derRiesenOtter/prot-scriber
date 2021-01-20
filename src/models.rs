use super::default::FILTER_REGEXS;
use eq_float::F64;
use regex::Regex;
use std::collections::HashMap;

pub fn overlap(sstart: i64, send: i64, slen: i64, qstart: i64, qend: i64, qlen: i64) -> F64 {
    F64((((qend - qstart + 1) + (send - sstart + 1)) as f64) / ((qlen + slen) as f64))
}

pub fn filter_stitle(stitle: &str, regexs: &Vec<Regex>) -> String {
    String::from(
        regexs
            .iter()
            .fold(stitle.to_string(), |accumulated, current| {
                current.replace_all(&accumulated, "").to_string()
            })
            .trim(),
    )
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Hit {
    pub id: String,
    pub bitscore: F64,
    pub sstart: i64,
    pub send: i64,
    pub slen: i64,
    pub overlap: F64,
    pub description: String,
}

impl Hit {
    pub fn new(
        id: &str,
        bitscore: &str,
        qstart: &str,
        qend: &str,
        qlen: &str,
        sstart: &str,
        send: &str,
        slen: &str,
        stitle: &str,
    ) -> Hit {
        let sstart_prsd = sstart.parse().unwrap();
        let send_prsd = send.parse().unwrap();
        let slen_prsd = slen.parse().unwrap();
        let qstart_prsd = qstart.parse().unwrap();
        let qend_prsd = qend.parse().unwrap();
        let qlen_prsd = qlen.parse().unwrap();
        Hit {
            id: String::from(id),
            bitscore: F64(bitscore.parse().unwrap()),
            sstart: sstart_prsd,
            send: send_prsd,
            slen: slen_prsd,
            overlap: overlap(
                sstart_prsd,
                send_prsd,
                slen_prsd,
                qstart_prsd,
                qend_prsd,
                qlen_prsd,
            ),
            description: filter_stitle(&stitle, &(*FILTER_REGEXS)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Query {
    id: String,
    hits: HashMap<String, Hit>,
}

impl Query {
    pub fn new(id: String, hits: HashMap<String, Hit>) -> Query {
        Query { id: id, hits: hits }
    }
}

trait HasDistanceMeasure {
    fn distance(to: &Hit) -> F64;
}

impl HasDistanceMeasure for Query {
    fn distance(to: &Hit) -> F64 {
        F64(4.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_regexs_extract_uni_prot_descriptions() {
        let t1 = "sp|C0LGP4|Y3475_ARATH Probable LRR receptor-like serine/threonine-protein kinase At3g47570 OS=Arabidopsis thaliana OX=3702 GN=At3g47570 PE=2 SV=1";
        assert_eq!(
            filter_stitle(t1, &(*FILTER_REGEXS)),
            "Probable LRR receptor-like serine/threonine-protein kinase At3g47570"
        );
    }

    #[test]
    fn parse_hit_from_strs() {
        let h1 = Hit::new("Hit_One",
            "123.4", "101", "200", "100", "1", "100", "100", 
            "sp|C0LGP4|Y3475_ARATH Probable LRR receptor-like serine/threonine-protein kinase At3g47570 OS=Arabidopsis thaliana OX=3702 GN=At3g47570 PE=2 SV=1");
        assert_eq!(h1.id, String::from("Hit_One"));
        assert_eq!(h1.bitscore, F64(123.4));
        assert_eq!(h1.sstart, 1);
        assert_eq!(h1.send, 100);
        assert_eq!(h1.slen, 100);
        assert_eq!(h1.overlap, F64(1.0));
        assert_eq!(
            h1.description,
            "Probable LRR receptor-like serine/threonine-protein kinase At3g47570"
        );
    }
}
