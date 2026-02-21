use std::{
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    abort::abort,
    error::{
        EXIT_CODE_FAILED_OPEN_FILE, EXIT_CODE_FAILED_OVERWRITE_CHECK,
        EXIT_CODE_FAILED_OVERWRITE_FORBIDDEN, EXIT_CODE_FAILED_READ_FILE,
        EXIT_CODE_FAILED_WRITE_PNG, FailureError,
    },
};

pub fn read(path: Option<&Path>) -> String {
    if let Some(path) = path {
        io::read_to_string(
            std::fs::File::open(path).unwrap_or_else(|err| {
                abort(&FailureError::OpenFile(err), EXIT_CODE_FAILED_OPEN_FILE)
            }),
        )
    } else {
        io::read_to_string(io::stdin())
    }
    .unwrap_or_else(|err| abort(&FailureError::ReadFile(err), EXIT_CODE_FAILED_READ_FILE))
}

pub fn pick_save_path(
    icon_path: Option<&Path>,
    save_path: Option<PathBuf>,
    overwrite: bool,
    forbid_ovewrite: bool,
) -> Option<PathBuf> {
    let final_ = save_path.or_else(|| {
        icon_path.map(ToOwned::to_owned).map(|mut p| {
            p.set_extension("png");
            p
        })
    });

    if !overwrite
        && let Some(path) = final_.as_ref()
        && std::fs::exists(path).unwrap_or_else(|err| {
            abort(
                &FailureError::Overwrite(err),
                EXIT_CODE_FAILED_OVERWRITE_CHECK,
            )
        })
    {
        if forbid_ovewrite {
            abort(
                &FailureError::OverwriteForbidden,
                EXIT_CODE_FAILED_OVERWRITE_FORBIDDEN,
            );
        }

        eprint!("{} already exists, overwrite? [y/N] ", path.display());

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap_or_else(|err| {
            abort(
                &FailureError::Overwrite(err),
                EXIT_CODE_FAILED_OVERWRITE_CHECK,
            )
        });

        if !["y\n", "yes\n"].contains(&buf.to_ascii_lowercase().as_str()) {
            abort(
                &FailureError::OverwriteForbidden,
                EXIT_CODE_FAILED_OVERWRITE_FORBIDDEN,
            );
        }
    }

    final_
}

pub fn save(path: Option<&Path>, buf: &[u8], buf_size: (usize, usize)) {
    let writer = BufWriter::new(if let Some(file) = path {
        Box::new(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file)
                .unwrap_or_else(|err| {
                    abort(&FailureError::OpenFile(err), EXIT_CODE_FAILED_OPEN_FILE)
                }),
        ) as Box<dyn Write>
    } else {
        Box::new(std::io::stdout()) as Box<dyn Write>
    });

    let mut encoder = png::Encoder::new(
        writer,
        buf_size.0.try_into().unwrap(),
        buf_size.1.try_into().unwrap(),
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut png_writer = encoder.write_header().unwrap_or_else(|err| {
        abort(&FailureError::WritePng(err), EXIT_CODE_FAILED_WRITE_PNG);
    });

    png_writer.write_image_data(buf).unwrap_or_else(|err| {
        abort(&FailureError::WritePng(err), EXIT_CODE_FAILED_WRITE_PNG);
    });
}
