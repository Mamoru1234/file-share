use std::{io, path::Path};

use actix_files::Directory;
use actix_web::{HttpRequest, dev::ServiceResponse};
use askama::Template;
use askama_actix::TemplateToResponse;

struct ListingItem {
  is_dir: bool,
  name: String,
  path: String,
}

#[derive(Template)]
#[template(path = "listing.html")]
struct ListingTemplate {
  path: String,
  items: Vec<ListingItem>
}

pub fn listing_renderer(dir: &Directory,
  req: &HttpRequest) -> Result<ServiceResponse, io::Error> {
  let request = req.clone();
  let base = Path::new(req.path());
  let items: Vec<ListingItem> = dir.path.read_dir()?
    .filter_map(|entry| {
      if !dir.is_visible(&entry) {
        return None;
      }
      Some(entry.unwrap())
    })
    .filter_map(|entry| {
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
    }).collect();
  let response = ListingTemplate {
    path: dir.path.to_string_lossy().to_string(),
    items
  }.to_response();
  Ok(ServiceResponse::new(request, response))
}

