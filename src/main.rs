mod csv_parser;

fn main() {
        let mapa_equivalencias = csv_parser::logic::get_equivalencias();
        print!("\x1B[2J\x1B[1;1H"); 
        println!("{:?}",mapa_equivalencias.get("SMA0501 - Cálculo I (BSI)"));

}
