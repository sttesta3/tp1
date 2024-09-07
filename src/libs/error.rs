pub static OPERACION_INVALIDA: u32 = 1; 
pub static DELETE_MAL_FORMATEADO: u32 = 2; 
pub static INSERT_MAL_FORMATEADO: u32 = 3; 
pub static SELECT_MAL_FORMATEADO: u32 = 4; 
pub static UPDATE_MAL_FORMATEADO: u32 = 5; 
pub static ARCHIVO_NO_PUDO_SER_ABIERTO: u32 = 6; 
pub static ARCHIVO_SIN_COLUMNAS: u32 = 7; 

pub fn print_err(error_code: u32) {
    // TODO print error    
}