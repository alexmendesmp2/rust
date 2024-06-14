use std::net::TcpStream;
use std::io::{Read, Write};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};

extern crate oracle;
use oracle::{Connection};

#[no_mangle]
pub extern "C" fn call_hsm( 
    module: *const c_char, 
    exponent: *const c_char,
    public_key: *mut c_char,
    private_key: *mut c_char,
    return_code: *mut c_int,
    return_reason: *mut c_int) -> *mut c_char {
    

    let hsm_address_str = "180.0.0.0";
    let lmk_str = "001";
    let hsm_address_str_ref: &str = &hsm_address_str;
    let mut command = String::new();

    if private_key.is_null() {
        let module_str = unsafe { CStr::from_ptr(module) }.to_string_lossy().into_owned();
        let exponent_str = unsafe { CStr::from_ptr(exponent) }.to_string_lossy().into_owned();
    
        command = format!("EI{:02X}{}{:02X}{}{}U", module_str.len()/2, module_str, exponent_str.len()/2, exponent_str, lmk_str);
    } else {
        command = format!("0000EI00PK;2,12,01");
    }
    

    match TcpStream::connect((hsm_address_str_ref, 1500)) {

        Ok(mut stream) => {
            
            match stream.write_all(command.as_bytes()) {

                Ok(_) => {
                    println!("Comando enviado ao HSM.");
                }
                Err(_) => {
                    return CString::new("Erro ao enviar commando ao HSM.").unwrap().into_raw();
                }
            }


            let mut response = String::new();

            match stream.read_to_string(&mut response) {

                Ok(_) => {
                    let c_response = CString::new(response).unwrap();
                    c_response.into_raw()
                }
                Err(_) => {
                    return CString::new("Erro ao receber resposta do HSM.").unwrap().into_raw();
                }

            }
        }
        Err(_) => {
            return CString::new("Erro ao conectar ao HSM").unwrap().into_raw();
        }
    }


}
    #[no_mangle]
    pub extern "C" fn save_to_oracle(label: *const c_char, key: *const c_char) {

        let label_str = unsafe { CStr::from_ptr(label) }.to_string_lossy().into_owned();
        let key_str = unsafe { CStr::from_ptr(key) }.to_string_lossy().into_owned();

        //let connect_params = ConnectParams::from_tns("tns_entry_name").wallet("path/to/wallet", "wallet_password");

        //match Connection::connect_with_params(connect_params) {
        match Connection::connect("user", "pass", "string_de_conexao") {

            Ok(conn) => {
                
                let sql = format!("INSERT INTO chave_table (LABEL, Key) VALUES ('{}', '{}')", label_str, key_str);

                match conn.execute(&sql, &[]) {
                    Ok(_) => {
                        println!("Chave salva com sucesso na tabela Oracle.");
                    }
                    Err(_) => {
                        println!("Erro ao salvar chave na tabela Oracle [aqui vai a descricao do erro]");
                    }
                }
            }
            Err(_) => {
                println!("Erro ao conectar ao banco de dados Oracle: [aqui vai a descricao do erro]");
            }
        }

    }


