use std::{fs::DirEntry, io, path::Path};

use actix_files::Directory;
use actix_web::{dev::ServiceResponse, HttpRequest};
use askama_actix::{Template, TemplateToResponse};


struct ListingItem {
  is_dir: bool,
  name: String,
  path: String,
}

fn map_dir_entry_listing_item(entry: &DirEntry, dir: &Directory, base: &Path) -> Option<ListingItem> {
  let item_path = match entry.path().strip_prefix(&dir.path) {
    Ok(p) if cfg!(windows) => base.join(p).to_string_lossy().replace('\\', "/"),
    Ok(p) => base.join(p).to_string_lossy().into_owned(),
    Err(_) => return None,
  };
  let is_dir = entry.metadata().and_then(|metadata| { Ok(metadata.is_dir())}).unwrap_or_else(|_| false);
  Some(ListingItem {
    is_dir,
    name: entry.file_name().to_string_lossy().to_string(),
    path: item_path.to_string()
  })
}

#[derive(Template)]
#[template(path = "listing.html")]
struct ListingTemplate {
  path: String,
  items: Vec<ListingItem>
}

pub fn listing_renderer(dir: &Directory,
  req: &HttpRequest) -> Result<ServiceResponse, io::Error> {
  let base = Path::new(req.path());
  let items: Vec<ListingItem> = dir.path.read_dir()?
    .filter_map(|entry| {
      if !dir.is_visible(&entry) {
        return None;
      }
      Some(entry.unwrap())
    })
    .filter_map(|entry| {
      map_dir_entry_listing_item(&entry, dir, base)
    }).collect();
  let template = ListingTemplate {
    path: dir.path.to_string_lossy().to_string(),
    items
  };
  return Ok(ServiceResponse::new(req.clone(), template.to_response()));
}
