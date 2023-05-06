use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct NuevoVehiculo {
    #[validate(
        length(min = 1, message = "Marca requerida"),
    )]
    pub marca: String,
    #[validate(
        length(min = 1, message = "Modelo requerido"),
    )]
    pub modelo: String,
    #[validate(
        range(min = 0, message = "Año requerido"),
    )]
    pub año: i16,
    #[validate(
        length(min = 1, message = "Placa requerida"),
    )]
    pub numero_placa: String,
    #[validate(
        length(min = 1, message = "Nombre economico requerido"),
    )]
    pub nombre_economico: String,
    #[validate(
        length(min = 1, message = "Numero de tarjeta requerido"),
    )]
    pub numero_tarjeta: String,
    //pub status: String,
    //pub active: bool,
    //pub picture: String,
}
