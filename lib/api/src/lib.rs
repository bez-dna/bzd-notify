pub mod tokens {
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tokens_descriptor");

    tonic::include_proto!("bzd.notify.tokens");
}
