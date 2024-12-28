use std::sync::LazyLock;

use crate::APP_NAME;

pub mod sing_box_file;

static TMP_DIR:LazyLock<std::path::PathBuf> = LazyLock::new(||{
    let tmp_dir = dirs::template_dir().unwrap().join(APP_NAME);
    if !tmp_dir.exists(){
        std::fs::create_dir_all(&tmp_dir).unwrap();
    }
    tmp_dir
});

pub static DATA_DIR:LazyLock<std::path::PathBuf> = LazyLock::new(||{
    let data_dir = dirs::data_dir().unwrap().join(APP_NAME);
    if !data_dir.exists(){
        std::fs::create_dir_all(&data_dir).unwrap();
    }
    data_dir
});

