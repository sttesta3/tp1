pub static OPERACION_INVALIDA: u32 = 101;
pub static DELETE_MAL_FORMATEADO: u32 = 102;
pub static INSERT_MAL_FORMATEADO: u32 = 103;
pub static SELECT_MAL_FORMATEADO: u32 = 104;
pub static UPDATE_MAL_FORMATEADO: u32 = 105;
pub static WHERE_MAL_FORMATEADO: u32 = 110;
pub static ORDER_BY_MAL_FORMATEADO: u32 = 111;
pub static NO_WHERE: u32 = 112;
pub static FALTA_ARCHIVO: u32 = 200;
pub static ARCHIVO_NO_PUDO_SER_ABIERTO: u32 = 201;
pub static ARCHIVO_VACIO: u32 = 202;
pub static ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS: u32 = 301;
pub static ERROR_RENOMBRADO_ARCHIVO_TEMPORAL: u32 = 401;
pub static ERROR_INTERCAMBIANDO_TEMPORAL_POR_OFICIAL: u32 = 402;
pub static ERROR_OPERACION_INVALIDA_EN_EJECUCCION: u32 = 403;

pub static CODE_INVALID_TABLE: &str = "INVALID_TABLE";
pub static CODE_INVALID_COLUMN: &str = "INVALID_TABLE";
pub static CODE_INVALID_SYNTAX: &str = "INVALID_SYNTAX";
pub static CODE_ERROR: &str = "ERROR";

pub static TEXTO_OP_INV: &str = "OPERACION INVALIDA";
pub static TEXTO_DELETE_MF: &str = "DELETE MAL FORMATEADO";
pub static TEXTO_INSERT_MF: &str = "INSERT MAL FORMATEADO";
pub static TEXTO_SELECT_MF: &str = "SELECT MAL FORMATEADO";
pub static TEXTO_UPDATE_MF: &str = "UPDATE MAL FORMATEADO";
pub static TEXTO_WHERE_MF: &str = "WHERE MAL FORMATEADO";
pub static TEXTO_ORDERBY_MF: &str = "ORDER BY MAL FORMATEADO";
pub static TEXTO_NO_WHERE: &str = "OPERACION REQUIERE WHERE";
pub static TEXTO_FALTA_ARCHIVO: &str = "FALTA ARCHIVO EN QUERY";
pub static TEXTO_ARCHIVO_NOPEN: &str = "ARCHIVO NO PUDO SER ABIERTO";
pub static TEXTO_ARCHIVO_VACIO: &str = "ARCHIVO VACIO";
pub static TEXTO_ARCHIVO_NO_CONTIENE_COLUMNAS: &str =
    "EL ARCHIVO NO CONTIENE TODAS LAS COLUMNAS SOLICITADAS";
pub static TEXTO_ERROR_RENOMBRADO_TEMP: &str = "ERROR RENOMBRADO ARCHIVO TEMPORAL";
pub static TEXTO_ERROR_INTERCAMBIANDO_TEMP: &str = "ERROR INTERCAMBIANDO TEMPORAL POR OFICIAL";
pub static TEXTO_ERROR_OPERACION_INVALIDA_EN_EJECUCCION: &str = "OPERACION INVALIDA EN EJECUCCION";
pub static TEXTO_ERROR_DESCONOCIDO: &str = "ERROR DESCONOCIDO";

pub fn print_err(error_code: u32) {
    println!(
        "{}: {}",
        get_text_code(&error_code),
        get_description(&error_code)
    );
}

fn get_text_code(error_code: &u32) -> &str {
    if error_code / 100 == 1 {
        CODE_INVALID_SYNTAX
    } else if error_code / 100 == 2 {
        CODE_INVALID_TABLE
    } else if error_code / 100 == 3 {
        CODE_INVALID_COLUMN
    } else {
        CODE_ERROR
    }
}

fn get_description(error_code: &u32) -> &str {
    if error_code / 100 == 1 {
        get_description_syntax(error_code)
    } else if error_code / 100 == 2 {
        get_description_table(error_code)
    } else if error_code / 100 == 3 {
        get_description_column(error_code)
    } else {
        get_description_error(error_code)
    }
}

fn get_description_syntax(error_code: &u32) -> &str {
    if *error_code == OPERACION_INVALIDA {
        TEXTO_OP_INV
    } else if *error_code == DELETE_MAL_FORMATEADO {
        TEXTO_DELETE_MF
    } else if *error_code == INSERT_MAL_FORMATEADO {
        TEXTO_INSERT_MF
    } else if *error_code == SELECT_MAL_FORMATEADO {
        TEXTO_SELECT_MF
    } else if *error_code == UPDATE_MAL_FORMATEADO {
        TEXTO_UPDATE_MF
    } else if *error_code == WHERE_MAL_FORMATEADO {
        TEXTO_WHERE_MF
    } else if *error_code == ORDER_BY_MAL_FORMATEADO {
        TEXTO_ORDERBY_MF
    } else if *error_code == NO_WHERE {
        TEXTO_NO_WHERE
    } else {
        TEXTO_ERROR_DESCONOCIDO
    }
}

fn get_description_table(error_code: &u32) -> &str {
    if *error_code == FALTA_ARCHIVO {
        TEXTO_FALTA_ARCHIVO
    } else if *error_code == ARCHIVO_NO_PUDO_SER_ABIERTO {
        TEXTO_ARCHIVO_NOPEN
    } else if *error_code == ARCHIVO_VACIO {
        TEXTO_ARCHIVO_VACIO
    } else {
        TEXTO_ERROR_DESCONOCIDO
    }
}

fn get_description_column(error_code: &u32) -> &str {
    if *error_code == ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS {
        TEXTO_ARCHIVO_NO_CONTIENE_COLUMNAS
    } else {
        TEXTO_ERROR_DESCONOCIDO
    }
}

fn get_description_error(error_code: &u32) -> &str {
    if *error_code == ERROR_INTERCAMBIANDO_TEMPORAL_POR_OFICIAL {
        TEXTO_ERROR_INTERCAMBIANDO_TEMP
    } else if *error_code == ERROR_RENOMBRADO_ARCHIVO_TEMPORAL {
        TEXTO_ERROR_RENOMBRADO_TEMP
    } else if *error_code == ERROR_OPERACION_INVALIDA_EN_EJECUCCION {
        TEXTO_ERROR_OPERACION_INVALIDA_EN_EJECUCCION
    } else {
        TEXTO_ERROR_DESCONOCIDO
    }
}
