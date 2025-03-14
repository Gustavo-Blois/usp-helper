use csv::Reader;
use std::collections::HashMap;


#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Equivalencia {
    #[serde(rename = "Disciplina cursada ou que pretende cursar")]
    disc_cursada: String,
    #[serde(rename = "Disciplina do curso do aluno (para a qual precisa de equivalência)")]
    disc_do_curso: String,
}

pub fn get_equivalencias() -> HashMap<String, Vec<Vec<String>>> {
    let mut mapa_equivalencias: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let rdr = Reader::from_path("./assets/equivalencias.csv");

    for item in rdr.expect("Couldnt deserialize").deserialize() {
        let mut disciplinas_equivalentes: Vec<Vec<String>> = Vec::new();
        let mut combinacao_equivalente: Vec<String> = Vec::new();
        let r:Equivalencia = item.expect("Couldnt deserialize");
        if let Some((disc1,disc2)) = r.disc_cursada.split_once('+'){
            combinacao_equivalente.push(disc1.to_string());
            combinacao_equivalente.push(disc2.to_string());
            disciplinas_equivalentes.push(combinacao_equivalente);
            
        } else {
            combinacao_equivalente.push(r.disc_cursada);
            disciplinas_equivalentes.push(combinacao_equivalente);
        }
        if mapa_equivalencias.contains_key(&r.disc_do_curso){
            let mut novas_equivalencias:Vec<Vec<String>> = mapa_equivalencias.get(&r.disc_do_curso).unwrap().to_vec();
            for disciplina in disciplinas_equivalentes {
                novas_equivalencias.push(disciplina);
            }
            mapa_equivalencias.insert(r.disc_do_curso, novas_equivalencias.to_vec());

        }else {
            mapa_equivalencias.insert(r.disc_do_curso, disciplinas_equivalentes);
        }
    }
    for (disc_cursada, disc_equivalente) in &mapa_equivalencias {
        println!("{} ||| {:?}", disc_cursada, disc_equivalente);
    }
    return mapa_equivalencias;
}

