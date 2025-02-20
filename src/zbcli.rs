#[derive(derive_more::Display, PartialEq)]
pub enum ZbcliAsset {
    #[display(fmt = "zbcli-rpi-zero2w")]
    Rpi0,

    #[display(fmt = "zbcli-rpi-zero2w-hardware")]
    Rpi0Hardware,

    #[display(fmt = "zbcli-rpi4")]
    Rpi4,

    #[display(fmt = "zbcli-rpi4-hardware")]
    Rpi4Hardware,

    #[display(fmt = "zbcli-rpi5")]
    Rpi5,

    #[display(fmt = "zbcli-rpi5-hardware")]
    Rpi5Hardware,
}
