#[derive(derive_more::Display, PartialEq)]
pub enum ZbcliAsset {
    #[display(fmt = "zbcli-rpi4.zip")]
    Rpi4,

    #[display(fmt = "zbcli-rpi4-hardware.zip")]
    Rpi4Hardware,

    #[display(fmt = "zbcli-rpi5.zip")]
    Rpi5,

    #[display(fmt = "zbcli-rpi5-hardware.zip")]
    Rpi5Hardware,
}
