use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use validator::Validate;

// #[derive(Debug, Clone, Serialize, Deserialize, Validate, Default, Getters, Setters)]
// #[getset(get = "pub", set = "pub")]
// pub struct BlobUploadRequest {
//     #[serde(rename = "projectName")]
//     project_name: String,   //name of idea or user custom (from client)
//     #[serde(rename = "blobName")]
//     blob_name: String,      //name of file
//     #[serde(rename = "clientAddress")]
//     client_address: String, //wallet address of client
// }

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct BlobUploadResponse {
    #[serde(rename = "storedQuiltBlobs")]
    quilt_upload_response: QuiltUploadResponse,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
pub struct QuiltUploadResponse {
    #[serde(rename = "blobStoreResult")]
    blob_store_result: BlobStoreResult,
    #[serde(rename = "storedQuiltBlobs")]
    stored_quilt_blobs: Vec<StoredQuiltBlob>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct BlobStoreResult {
    #[serde(rename = "newlyCreated")]
    newly_created: NewlyCreated,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct NewlyCreated {
    #[serde(rename = "blobObject")]
    blob_object: BlobObject,
    #[serde(rename = "resourceOperation")]
    resource_operation: ResourceOperation,
    cost: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct BlobObject {
    id: String, // Sui object ID, e.g. "0x2b3b..."
    #[serde(rename = "registeredEpoch")]
    registered_epoch: u64,
    #[serde(rename = "blobId")]
    blob_id: String, // Walrus blob ID
    size: u64,
    #[serde(rename = "encodingType")]
    encoding_type: String, // e.g. "RS2"
    #[serde(rename = "certifiedEpoch")]
    certified_epoch: Option<u64>,
    storage: Storage,
    deletable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct Storage {
    id: String,
    #[serde(rename = "startEpoch")]
    start_epoch: u64,
    #[serde(rename = "endEpoch")]
    end_epoch: u64,
    #[serde(rename = "storageSize")]
    storage_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct ResourceOperation {
    #[serde(rename = "registerFromScratch")]
    register_from_scratch: RegisterFromScratch,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[set = "pub"]
#[get = "pub"]
struct RegisterFromScratch {
    #[serde(rename = "encodedLength")]
    encoded_length: u64,
    #[serde(rename = "epochsAhead")]
    epochs_ahead: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StoredQuiltBlob {
    identifier: String,
    #[serde(rename = "quiltPatchId")]
    quilt_patch_id: String,
    /// Một số response có field range (ví dụ khi blob nằm ở vị trí cụ thể trong quilt)
    /// Nếu không có thì sẽ là None
    range: Option<Vec<u64>>,
}
