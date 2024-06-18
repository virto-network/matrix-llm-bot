use ruma::MxcUri;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

pub fn mxc_to_download_uri(uri: &MxcUri) -> Option<String> {
    match uri.parts() {
        Ok((server, id)) => {
            let uri = format!(
                "https://matrix-client.matrix.org/_matrix/media/v3/download/{}/{}",
                server, id
            );
            Some(uri)
        }
        Err(_) => None,
    }
}