use thiserror::Error;
use aws_sdk_s3::{Client, types::{SdkError, ByteStream}, error::PutObjectError, output::PutObjectOutput};
use std::{path::{Path, PathBuf}, collections::HashSet, fs};
use log::{debug, warn, error, info};
use anyhow::Result;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("File upload failed")]
    UploadFailed(#[from] SdkError<PutObjectError>),

    #[error("Could not parse path")]
    InvalidPath,
}

pub type UploadResult<T> = Result<T, UploadError>;

pub struct S3Client {
    s3_client: Client,
    bucket: String
}

impl S3Client {
    pub async fn new(
        bucket: String
    ) -> UploadResult<S3Client> {
        let aws_config = aws_config::load_from_env().await;
        let client = Client::new(&aws_config);

        Ok(S3Client {
            s3_client: client,
            bucket
        })
    }

    pub async fn upload_file(&self, data: ByteStream, key: &str) -> UploadResult<PutObjectOutput> {
        let upload_response = self
            .s3_client
            .put_object()
            .bucket(&self.bucket)
            .key(key.replace("\\", "/"))
            .body(data)
            .send()
            .await;

        match upload_response {
            Ok(output) => Ok(output),
            Err(err) => Err(UploadError::UploadFailed(err)),
        }
    }
}

fn expand_path(input: PathBuf) -> UploadResult<PathBuf> {
    let expanded_path = shellexpand::tilde(&parse_path(input)?).to_string();
    Ok(Path::new(&expanded_path).to_owned())
}

fn split_filename(filename: &str) -> Vec<String> {
    filename.split(&['/', '\\'][..]).map(|s| s.to_string()).collect()
}

fn parse_path(path: PathBuf) -> UploadResult<String> {
    path.into_os_string().into_string().map_err(|_| UploadError::InvalidPath)
}

fn strip_path(path: &Path, root: &Path) -> Option<String> {
    let path = match path.strip_prefix(root) {
        Ok(p) => match p.to_str() {
            Some(p) => p,
            None => {
                error!("Failed to parse path: {:?}", path);
                return None;
            }
        },
        Err(err) => {
            error!("Failed to parse path: {:?}: {}", path, err);
            return None
        }
    };
    Some(path.to_owned())
}

async fn traverse_directories(root: &Path, existing_files: &mut HashSet<Vec<String>>, client: &S3Client) -> UploadResult<()> {
    let mut paths = vec![root.to_owned()];

    while !paths.is_empty() {
        let path = paths.remove(0);
        debug!("Diving into directory: {:?}", path);

        let metadata = match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(err) => {
                warn!("unable to read metadata for {:?}: {}", &path, err);
                continue;
            }
        };

        if metadata.is_file() {
            debug!("Processing {:?}", path.file_name());
            let stripped_path = match strip_path(&path, root) {
                Some(p) => p,
                None => continue,
            };

            let filename_segments = split_filename(&stripped_path);

            if existing_files.contains(&filename_segments) {
                info!("Skippinf existing file: {}", stripped_path);
                continue;
            }

            info!("Uploading new file: {}", stripped_path);
            existing_files.insert(filename_segments);

            match ByteStream::from_path(path).await {
                Ok(d) => {
                    client.upload_file(d, stripped_path.as_ref()).await?;
                }
                Err(err) => {
                    error!("Failed to read file {:?}: {}", stripped_path, err)
                }
            }
            continue;
        }

        for entry in fs::read_dir(path).unwrap() {
            let directory = entry.unwrap();
            paths.push(directory.path())
        }

    }

    Ok(())
}

pub async fn handle_upload(path: &PathBuf) {
    let root = expand_path(path.to_owned()).unwrap_or_else(|err| panic!("Failed to read root path: {}", err));
    let client = S3Client::new(
        "dekube".to_string()
    ).await.unwrap_or_else(|err| panic!("Unable to establish a connection {}", err));

    match traverse_directories(&root, &mut HashSet::new(), &client).await {
        Ok(()) => info!("All directories synced"),
        Err(err) => error!("Failed to sync directories: {}", err),
    }
}
