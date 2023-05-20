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

#[derive(Default)]
struct SP {
    cnes: Option<u32>,
    aa: Option<u16>,
    mm: Option<u8>,
    atoprof: Option<String>,
    valato: Option<f32>,
    qtd_ato: Option<u16>,
    procrea: Option<String>,
}

fn read_dbf() {
    let mut reader =
        dbase::Reader::from_path("./src/test.dbf").expect("Erro na leitura do arquivo ");
    let records = reader.read().expect("Erro na leitura DBF");
    //let records = reader.read().expect("Erro na leitura DBF");
    println!("Tamanho: {}", records.len());
    let mut soma = 0.0;
    for record in records {
        let mut sp = SP::default();
        for (name, value) in record {
            match value {
                FieldValue::Character(Some(x)) => {
                    //println!("Character: {}", character_value);
                    match name.as_str() {
                        "SP_PROCREA" => sp.procrea = Some(x),
                        "SP_CNES" => sp.cnes = x.parse().ok(),
                        "SP_ATOPROF" => sp.atoprof = x.parse().ok(),
                        "SP_AA" => sp.aa = x.parse().ok(),
                        "SP_MM" => sp.mm = x.parse().ok(),
                        _ => {}
                    }
                }
                FieldValue::Numeric(Some(x)) => match name.as_str() {
                    "SP_QTD_ATO" => sp.qtd_ato = Some(x as u16),
                    "SP_VALATO" => sp.valato = Some(x as f32),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    println!("soma final: {:?}", soma);
}
