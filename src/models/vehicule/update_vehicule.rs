use serde::{Serialize, Deserialize};
use validator::Validate;

use super::EstadoVehiculo;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct ActualizaVehiculo{
    #[validate(
        length(min = 1, message = "Marca requerida"),
    )]
    pub marca: Option<String>,
    #[validate(
        length(min = 1, message = "Modelo requerido"),
    )]
    pub modelo: Option<String>,
    #[validate(
        range(min = 0, message = "Año requerido"),
    )]
    pub año: Option<i16>,
    #[validate(
        length(min = 1, message = "Numero de placa requerido"),
    )]
    pub numero_placa: Option<String>,
    #[validate(
        length(min = 1, message = "Nombre economico requerido"),
    )]
    pub nombre_economico: Option<String>,
    #[validate(
        length(min = 1, message = "Numero de tarjeta requerido"),
    )]
    pub numero_tarjeta: Option<String>,
    //#[validate(
    //    custom(function = "validate_status")
    //)]
    //pub estado: Option<String>,
    pub estado: Option<EstadoVehiculo>,
    pub activo: Option<bool>,
}
