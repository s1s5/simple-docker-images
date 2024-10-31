use anyhow::Result;
use clap::Parser;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use dropbox_sdk::files;
use dropbox_sdk::oauth2::Authorization;
use log::{debug, error, info};
use rand::Rng as _;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

/// The size of a block. This is a Dropbox constant, not adjustable.
const BLOCK_SIZE: usize = 4 * 1024 * 1024;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "-")]
    src_file: String,

    #[arg(short = 'd', long)]
    target_path: String,

    #[arg(long)]
    suffix: Option<String>,

    #[arg(short = 't', long)]
    dropbox_token: Option<String>,

    #[arg(short = 'e', long)]
    dropbox_token_envvar: Option<String>,

    #[arg(long)]
    overwrite: bool,

    #[arg(long, default_value = "2")]
    blocks_per_request: usize,
}

/// This function does it all.
fn upload_file(
    client: UserAuthDefaultClient,
    mut source_file: impl std::io::Read,
    dest_path: String,
    chunk_size: usize,
    overwrite: bool,
) -> Result<()> {
    let session_id = files::upload_session_start(
        &client,
        &files::UploadSessionStartArg::default()
            .with_session_type(files::UploadSessionType::Sequential),
        &[],
    )??
    .session_id;

    info!("upload session ID is {session_id}");

    let mut offset = 0;
    let mut buffer = vec![0; chunk_size];
    loop {
        let mut read = 0;
        while read < chunk_size {
            let r = source_file.read(&mut buffer[read..])?;
            if r == 0 {
                break;
            }
            read += r;
        }
        if read == 0 {
            break;
        }
        debug!("uploading offset={offset}, bytes={read}");

        let append_arg = files::UploadSessionAppendArg::new(files::UploadSessionCursor::new(
            session_id.clone(),
            offset,
        ));

        upload_block_with_retry(&client, append_arg, &buffer[..read])?;
        debug!("uploaded!");

        offset += read as u64;
    }

    debug!("upload data finished.");
    let commit_info = files::CommitInfo::new(dest_path);
    let commit_info = if overwrite {
        commit_info.with_mode(files::WriteMode::Overwrite)
    } else {
        commit_info.with_mode(files::WriteMode::Add)
    };
    let commit_arg = files::UploadSessionFinishArg::new(
        files::UploadSessionCursor::new(session_id.clone(), offset),
        commit_info,
    );
    debug!("fnishing upload");
    let mut retry = 0;
    while retry < 3 {
        match files::upload_session_finish(&client, &commit_arg, &[]) {
            Ok(Ok(file_metadata)) => {
                info!("Upload succeeded!");
                info!("{:#?}", file_metadata);
                break;
            }
            Ok(Err(dropbox_sdk::files::UploadSessionFinishError::Path(
                dropbox_sdk::files::WriteError::Conflict(p),
            ))) => {
                error!("Error finishing upload: Write Conflict {p:?}");
                Err(p)?
            }
            err => {
                error!("Error finishing upload: {:?}", err);
                retry += 1;
                sleep(Duration::from_secs(1));
            }
        }
    }
    Ok(())
}

/// Upload a single block, retrying a few times if an error occurs.
///
/// Prints progress and upload speed, and updates the UploadSession if successful.
fn upload_block_with_retry(
    client: &UserAuthDefaultClient,
    arg: files::UploadSessionAppendArg,
    buf: &[u8],
) -> Result<()> {
    let mut errors = 0;
    loop {
        match files::upload_session_append_v2(client, &arg, buf) {
            Ok(Ok(())) => break,
            Err(dropbox_sdk::Error::RateLimited {
                reason,
                retry_after_seconds,
            }) => {
                info!(
                    "rate-limited ({}), waiting {} seconds",
                    reason, retry_after_seconds
                );
                if retry_after_seconds > 0 {
                    sleep(Duration::from_secs(u64::from(retry_after_seconds)));
                }
                continue;
            }
            error => {
                errors += 1;
                if errors < 3 {
                    continue;
                }
                return Err(anyhow::anyhow!(
                    "Error calling upload_session_append: {error:?}"
                ));
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let source_file: Box<dyn std::io::Read> = if args.src_file == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(&args.src_file)?)
    };

    let token = if let Some(token) = args.dropbox_token {
        token
    } else if let Some(envvar) = args.dropbox_token_envvar {
        std::env::var(&envvar).expect("dropbox token env not found")
    } else {
        panic!("set --dropbox-token or --dropbox-token-envvar")
    };

    // TODO: そのうちこのトークンは使えなくなるらしいので注意
    #[allow(deprecated)]
    let auth = Authorization::from_long_lived_access_token(token);
    let client = UserAuthDefaultClient::new(auth);

    let target_path = if let Some(suffix) = args.suffix {
        let mut rng = rand::thread_rng();

        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let rchar: String = (0..4)
            .map(|_| {
                let index = rng.gen_range(0..chars.len());
                chars.chars().nth(index).unwrap() // Get the character at the random index
            })
            .collect();

        let now = chrono::Utc::now().with_timezone(&chrono_tz::Japan);
        format!(
            "{}-{}-{rchar}{suffix}",
            args.target_path,
            now.format("%Y%m%d-%H%M%S")
        )
    } else {
        args.target_path
    };

    upload_file(
        client,
        source_file,
        target_path,
        args.blocks_per_request * BLOCK_SIZE,
        args.overwrite,
    )?;

    Ok(())
}
