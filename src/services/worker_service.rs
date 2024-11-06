use crate::db::DbPool;
use crate::error::AppError;
use crate::handlers::worker::FileSystemEntry;
use crate::models::worker::{NewWorker, UpdateWorker, Worker};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub struct WorkerService;

impl WorkerService {
    pub fn list_filesystem(input_path: PathBuf) -> Result<Vec<FileSystemEntry>, std::io::Error> {
        // Try multiple base paths
        let base_paths = vec![
            PathBuf::from("/home/worker"),
            PathBuf::from("/app"),
            PathBuf::from("/home/computeruse"),
        ];

        let mut base_path = None;
        for path in base_paths.iter() {
            log::info!("Checking base path: {:?}", path);
            if path.exists() {
                log::info!("Found existing base path: {:?}", path);
                base_path = Some(path);
                break;
            }
        }

        let base_path = base_path.ok_or_else(|| {
            log::error!("No valid base path found");
            std::io::Error::new(std::io::ErrorKind::NotFound, "No valid base path found")
        })?;

        let full_path = if input_path.has_root() {
            base_path.join(input_path.strip_prefix("/").unwrap_or(&input_path))
        } else {
            base_path.join(&input_path)
        };

        log::info!("Listing filesystem:");
        log::info!("Input path: {:?}", input_path);
        log::info!("Base path: {:?}", base_path);
        log::info!("Full path: {:?}", full_path);

        // Check if full path exists
        if !full_path.exists() {
            log::error!("Full path does not exist: {:?}", full_path);
            // If root path doesn't exist, try to create basic structure
            if input_path.as_os_str() == "/" {
                log::info!("Creating basic directory structure");
                let dirs = ["Documents", "Downloads", "Desktop"];
                for dir in dirs.iter() {
                    let dir_path = base_path.join(dir);
                    if !dir_path.exists() {
                        log::info!("Creating directory: {:?}", dir_path);
                        fs::create_dir_all(&dir_path)?;
                    }
                }
                // Create a test file
                let test_file = base_path.join("Documents/welcome.txt");
                if !test_file.exists() {
                    log::info!("Creating test file: {:?}", test_file);
                    fs::write(&test_file, "Welcome to the file system!\n")?;
                }
                // Try listing the base path again
                return Self::list_filesystem(input_path);
            }
            return Ok(Vec::new());
        }

        // Check if path is a directory
        if !full_path.is_dir() {
            log::error!("Path is not a directory: {:?}", full_path);
            return Ok(Vec::new());
        }

        if !full_path.starts_with(base_path) {
            log::error!(
                "Attempted to access path outside base directory: {:?}",
                full_path
            );
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();

        // Add parent directory entry if not at root
        if full_path != *base_path {
            let parent_path = full_path
                .parent()
                .and_then(|p| p.strip_prefix(base_path).ok())
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| String::from("/"));

            log::info!("Adding parent directory entry with path: {}", parent_path);

            entries.push(FileSystemEntry {
                name: String::from(".."),
                path: parent_path,
                is_dir: true,
                size: 0,
                modified: Utc::now().to_rfc3339(),
            });
        }

        // List directory contents
        match fs::read_dir(&full_path) {
            Ok(dir_entries) => {
                for entry in dir_entries {
                    match entry {
                        Ok(entry) => {
                            match entry.metadata() {
                                Ok(metadata) => {
                                    let name = entry.file_name().to_string_lossy().into_owned();

                                    // Skip hidden files
                                    if name.starts_with('.') {
                                        log::debug!("Skipping hidden file: {}", name);
                                        continue;
                                    }

                                    let path = match entry.path().strip_prefix(base_path) {
                                        Ok(rel_path) => rel_path.to_string_lossy().into_owned(),
                                        Err(e) => {
                                            log::error!("Failed to strip base path: {}", e);
                                            continue;
                                        }
                                    };

                                    let modified = metadata
                                        .modified()
                                        .ok()
                                        .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
                                        .map(|d| {
                                            DateTime::<Utc>::from_timestamp(d.as_secs() as i64, 0)
                                        })
                                        .flatten()
                                        .unwrap_or_else(|| Utc::now());

                                    log::info!(
                                        "Found entry - name: {}, path: {}, is_dir: {}",
                                        name,
                                        path,
                                        metadata.is_dir()
                                    );

                                    entries.push(FileSystemEntry {
                                        name,
                                        path,
                                        is_dir: metadata.is_dir(),
                                        size: metadata.len(),
                                        modified: modified.to_rfc3339(),
                                    });
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to get metadata for {:?}: {}",
                                        entry.path(),
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to read directory entry: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read directory {:?}: {}", full_path, e);
            }
        }

        log::info!("Found {} entries in {:?}", entries.len(), full_path);
        Ok(entries)
    }

    pub fn get_file_content(input_path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
        // Try multiple base paths
        let base_paths = vec![
            PathBuf::from("/home/worker"),
            PathBuf::from("/app"),
            PathBuf::from("/home/computeruse"),
        ];

        let mut base_path = None;
        for path in base_paths.iter() {
            log::info!("Checking base path: {:?}", path);
            if path.exists() {
                log::info!("Found existing base path: {:?}", path);
                base_path = Some(path);
                break;
            }
        }

        let base_path = base_path.ok_or_else(|| {
            log::error!("No valid base path found");
            std::io::Error::new(std::io::ErrorKind::NotFound, "No valid base path found")
        })?;

        let full_path = if input_path.has_root() {
            base_path.join(input_path.strip_prefix("/").unwrap_or(&input_path))
        } else {
            base_path.join(&input_path)
        };

        log::info!("Reading file:");
        log::info!("Input path: {:?}", input_path);
        log::info!("Base path: {:?}", base_path);
        log::info!("Full path: {:?}", full_path);

        if !full_path.exists() {
            log::error!("File does not exist: {:?}", full_path);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        }

        if !full_path.starts_with(base_path) {
            log::error!(
                "Attempted to access file outside base directory: {:?}",
                full_path
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Access denied",
            ));
        }

        fs::read(full_path)
    }

    pub fn create_worker(pool: &DbPool, new_worker: NewWorker) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new worker into database: {:?}", new_worker);
        match diesel::insert_into(workers)
            .values(&new_worker)
            .get_result(conn)
        {
            Ok(worker) => {
                log::info!("Worker inserted into database: {:?}", worker);
                Ok(worker)
            }
            Err(e) => {
                log::error!("Error inserting worker into database: {:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn list_workers(pool: &DbPool, user_id: Uuid) -> Result<Vec<Worker>, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        workers
            .filter(user_id.eq(user_id))
            .load::<Worker>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        workers
            .filter(id.eq(worker_id).and(user_id.eq(user_id)))
            .first(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_worker(
        pool: &DbPool,
        worker_id: Uuid,
        update_data: UpdateWorker,
        user_id: Uuid,
    ) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub fn activate_worker(
        pool: &DbPool,
        worker_id: Uuid,
        user_id: Uuid,
    ) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(active.eq(true))
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn deactivate_worker(
        pool: &DbPool,
        worker_id: Uuid,
        user_id: Uuid,
    ) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(active.eq(false))
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }
}
