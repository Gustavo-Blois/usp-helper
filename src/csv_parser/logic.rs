use csv::Reader;
use serde::Deserialize;
struct Disciplina {
    nome: String,
    codigo: String,
}

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Equivalencia {
    #[serde(rename = "Disciplina cursada ou que pretende cursar")]
    disc_cursada: String,
    #[serde(rename = "Disciplina do curso do aluno (para a qual precisa de equivalência)")]
    disc_do_curso:String,
}

pub fn get_equivalencias() {
    let rdr = Reader::from_path("../../assets/equivalencias.csv");
    let mut iter = rdr.expect("Não foi possivel desserializar o item").into_deserialize();
    for item in iter{
        let record: Equivalencia = item.unwrap();
        println!("{:?}",record)
    }

    
}
