use getset::{Getters, Setters};

#[derive(Getters, Setters, Default)]
#[getset(set = "pub", get = "pub")]
pub struct BlobInfomation {
    client_address: String, //wallet address of client
    project_name: String,   //name of idea or user custom (from client)
    blob_name: String,      //name of file
    blob_father_path: String, //path of blob
}
