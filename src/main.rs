mod merge;
mod path_utils;

use std::path::Path;
use lopdf::Document;
use clap::{Arg, App};

use path_utils::{unwrap_filestem, prefix_groups, extension_matches_in_dir};
use merge::{merge_documents_from_paths, merge_documents, pad_odd_paged_document};

fn main() {

    let matches = App::new("Notebas Tools")
        .version("0.1.0")
        .about("Tools to make the #notebaslife smooth")
        .arg(Arg::with_name("dir")
            .help("Path to directory to get pdf documents from")
            .required(true))
        .arg(Arg::with_name("padding")
            .help("Path to document to use as padding")
            .required(true))
        .arg(Arg::with_name("unmerged")
            .short("u")
            .long("unmerged")
            .help("Pad documents without merging them into one"))
        .get_matches();

    let out_dir = Path::new(matches.value_of("dir").unwrap());
    let padding_path = Path::new(matches.value_of("padding").unwrap());

    let mut local_documents = extension_matches_in_dir(out_dir, "pdf");
    local_documents.sort_by(|p1,p2| unwrap_filestem(p1).to_lowercase().cmp(&unwrap_filestem(p2).to_lowercase()));

    let document_groups = prefix_groups(&local_documents);

    let merged_documents: Vec<(String, Document)> = document_groups.into_iter().map(|(prefix, document_group)| (prefix, merge_documents_from_paths(&document_group))).collect();
    let padded_documents: Vec<(String, Document)> = merged_documents.into_iter().map(|(prefix, merged_document)| (prefix, pad_odd_paged_document(merged_document, padding_path))).collect();

    if matches.is_present("unmerged") {
        for (prefix, mut document) in padded_documents {
            let filename = format!("{}_padded.pdf", prefix);
            let file_path = out_dir.join(&filename);
            if file_path.exists() {
                std::fs::remove_file(&file_path).expect("Error removing existing pdf");
            }
            document.save(&file_path).unwrap();
        }
    } else {
        let mut lefse = merge_documents(padded_documents.into_iter().map(|t| t.1).collect());
        let lefse_path = out_dir.join("lefse.pdf");
        if lefse_path.exists() {
            std::fs::remove_file(&lefse_path).expect("Error removing existing lefse.pdf");
        }
        lefse.save(&lefse_path).unwrap();
    }
}
