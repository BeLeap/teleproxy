pub mod teleproxy {
    tonic::include_proto!("teleproxy");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("teleproxy_descriptor");
}
