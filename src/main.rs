use serde_derive::Deserialize;

use core::num;
use dbase::{FieldValue, Record};
use ftp::FtpStream;
use std::fs::File;
use std::io::copy;
use std::process::Command;

struct AIH {}

fn main() {
    //ftp_download();
    //dbc_decompression();
    read_dbf();
}

fn ftp_download() {
    let mut ftp_stream =
        FtpStream::connect("ftp.datasus.gov.br:21").unwrap_or_else(|err| panic!("{}", err));

    ftp_stream
        .login("anonymous", "anonymous@")
        .unwrap_or_else(|err| panic!("{}", err));

    ftp_stream
        .cwd("/dissemin/publicos/SIHSUS/200801_/Dados")
        .unwrap_or_else(|err| panic!("{}", err));

    ftp_stream.retr("SPCE1001.dbc", |stream| {
        let mut file =
            File::create("./src/test.dbc").map_err(|e| ftp::FtpError::ConnectionError((e)))?;
        copy(stream, &mut file).map_err(|e| ftp::FtpError::ConnectionError(e))
    });

    let _ = ftp_stream.quit();
}

fn dbc_decompression() {
    Command::new("/home/baca/Documents/devo/blast-dbf/main")
        .args(["test.dbc", "test.dbf"])
        .spawn()
        .expect("erro");
}

struct SP {
    sp_cnes: u32,
    sp_aa: u16,
    sp_mm: u8,
    sp_atoprof: String,
    sp_valato: f32,
    sp_qtd_ato: u16,
    sp_procrea: String,
}

fn read_dbf() {
    let mut reader =
        dbase::Reader::from_path("./src/test.dbf").expect("Erro na leitura do arquivo ");
    let records = reader.read().expect("Erro na leitura DBF");
    //let records = reader.read().expect("Erro na leitura DBF");
    println!("Tamanho: {}", records.len());
    let mut soma = 0.0;
    for record in records {
        for (name, value) in record {
            let mut sp_cnes = "";
            let mut sp_aa = "";
            let mut sp_mm = "";
            let mut sp_atoprof = "";
            let mut sp_valato = 0.0;
            let mut sp_qtd_ato = 0.0;
            let mut sp_procrea = "";

            match value {
                FieldValue::Character(Some(character_value)) => {
                    //println!("Character: {}", character_value);
                    match name.as_str() {
                        "SP_CNES" => sp_cnes = &character_value,
                        "SP_ATOPROF" => sp_atoprof = &character_value,
                        "SP_PROCREA" => sp_procrea = &character_value,
                        "SP_AA" => sp_aa = &character_value,
                        "SP_MM" => sp_mm = &character_value,
                        _ => {}
                    }
                }
                FieldValue::Numeric(Some(numeric_value)) => match name.as_str() {
                    "SP_QTD_ATO" => sp_qtd_ato = numeric_value,
                    "SP_VALATO" => sp_valato = numeric_value,
                    _ => {}
                },
                _ => {}
            }
            let sp = SP {
                sp_cnes: sp_cnes.parse::<u32>().unwrap(),
                sp_atoprof: sp_atoprof.to_string(),
                sp_procrea: sp_procrea.to_string(),
                sp_aa: sp_aa.parse::<u16>().unwrap(),
                sp_mm: sp_mm.parse::<u8>().unwrap(),
                sp_qtd_ato: sp_qtd_ato as u16,
                sp_valato: sp_valato as f32,
            };
        }
    }
    println!("soma final: {:?}", soma);
}
