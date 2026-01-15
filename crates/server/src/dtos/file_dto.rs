use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, Setters, Default, Debug)]
#[getset(set = "pub", get = "pub")]
pub struct BlobInfomation {
    client_address: String,   //wallet address of client
    identifier: String,       //name of idea or user custom (from client)
    blob_name: String,        //name of file
    blob_father_path: String, //path of blob
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct RequestUpload {
    prototypeCode: String,
    identifier: String,
    uploadId: String,
}
