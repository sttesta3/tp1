pub static OPERACION_INVALIDA: u32 = 1; 
pub static DELETE_MAL_FORMATEADO: u32 = 2; 
pub static INSERT_MAL_FORMATEADO: u32 = 3; 
pub static SELECT_MAL_FORMATEADO: u32 = 4; 
pub static UPDATE_MAL_FORMATEADO: u32 = 5; 
pub static ARCHIVO_NO_PUDO_SER_ABIERTO: u32 = 6; 
pub static ARCHIVO_VACIO: u32 = 7; 
pub static ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS: u32 = 8; 
pub static WHERE_EN_DELETE_MAL_FORMATEADO: u32 = 9; 
pub static WHERE_MAL_FORMATEADO: u32 = 10; 
pub static ORDER_BY_MAL_FORMATEADO: u32 = 11; 
pub static NO_WHERE: u32 = 12; 

pub fn print_err(error_code: u32) {
    // TODO print error    
}